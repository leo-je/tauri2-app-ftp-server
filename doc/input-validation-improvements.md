# 输入验证和清理改进文档

## 概述

本次更新为 FTP 服务器应用添加了全面的输入验证和清理机制，防止路径遍历、注入攻击等安全风险。

## 改进内容

### 1. 后端 Rust 验证（`src-tauri/src/invoke_command.rs`）

#### 1.1 路径验证 (`sanitize_path`)

**功能：**
- 去除首尾空白字符
- 检查路径长度限制（最大 4096 字符）
- 检测并阻止危险模式：
  - `../` 和 `..\\` - 防止路径遍历攻击
  - `\0` - 防止空字节注入
  - `<`, `>`, `|`, `*`, `?` - 防止 shell 注入
- 使用 `canonicalize()` 规范化路径
- 验证路径是否存在且为目录

**示例：**
```rust
let path = sanitize_path("/home/user/ftp")?;
// 返回规范化的绝对路径
```

**阻止的攻击：**
- 路径遍历：`../../../etc/passwd`
- 空字节注入：`/safe/path\0/../../etc/passwd`
- Shell 注入：`/path | rm -rf /`

#### 1.2 端口验证 (`sanitize_port`)

**功能：**
- 去除首尾空白
- 验证端口范围（1-65535）
- 警告特权端口使用（< 1024）
- 警告标准 FTP 端口（21）

**示例：**
```rust
let port = sanitize_port("2121")?;  // Ok(2121)
let port = sanitize_port("21")?;    // Ok(21) + 警告
```

#### 1.3 用户名验证 (`sanitize_username`)

**功能：**
- 去除首尾空白
- 长度限制（1-64 字符）
- 字符白名单：仅允许字母、数字、下划线、短横线
- 防止注入攻击

**示例：**
```rust
let username = sanitize_username("admin_123")?;  // Ok
let username = sanitize_username("admin;drop")?; // Err - 包含非法字符
```

#### 1.4 密码验证 (`sanitize_password`)

**功能：**
- 长度限制（1-128 字符）
- 弱密码检测（仅警告，不强制）
- 检测常见弱密码：`123456`, `password`, `admin`, `root`, `12345678`

**示例：**
```rust
let password = sanitize_password("StrongP@ss123")?;  // Ok
let password = sanitize_password("123456")?;         // Ok + 警告
```

#### 1.5 文件权限验证 (`sanitize_file_auth`)

**功能：**
- 去除空白并转大写
- 仅允许 "R"（只读）或 "W"（读写）

**示例：**
```rust
let auth = sanitize_file_auth("w")?;  // Ok("W")
let auth = sanitize_file_auth("x")?;  // Err
```

#### 1.6 用户列表验证 (`sanitize_users_json`)

**功能：**
- 解析 JSON 格式
- 用户数量限制（最多 100 个）
- 递归验证每个用户的字段
- 清理所有用户输入数据

**示例：**
```rust
let users = sanitize_users_json(r#"[{"username":"admin","password":"123"}]"#)?;
```

### 2. 前端 TypeScript 验证（`src/utils/validation.ts`）

#### 2.1 验证函数

提供与后端一致的验证逻辑：

- `validatePath()` - 路径验证
- `validatePort()` - 端口验证
- `validateUsername()` - 用户名验证
- `validatePassword()` - 密码验证
- `validateFileAuth()` - 文件权限验证
- `validateUser()` - 单个用户验证
- `validateUsers()` - 用户列表验证

**返回格式：**
```typescript
{
  valid: boolean;
  error?: string;      // 错误消息
  warning?: string;    // 警告消息
  errors?: string[];   // 多个错误
  warnings?: string[]; // 多个警告
}
```

#### 2.2 前端集成

**Ftp.vue 中的使用：**
```typescript
// 验证路径
const pathValidation = validatePath(dirPath.value);
if (!pathValidation.valid) {
    ElMessage({ type: "error", message: pathValidation.error });
    return;
}

// 验证端口
const portValidation = validatePort(port.value);
if (!portValidation.valid) {
    ElMessage({ type: "error", message: portValidation.error });
    return;
}

// 显示警告
if (portValidation.warning) {
    ElMessage({ type: "warning", message: portValidation.warning });
}
```

**FtpAuth.vue 中的使用：**
```typescript
const validation = validateUser({
    username: form.username,
    password: form.password,
    fileAuth: form.fileAuth
});

if (validation.errors.length > 0) {
    ElMessage({ type: 'error', message: validation.errors.join('；') });
    return;
}

// 显示警告但允许继续
if (validation.warnings.length > 0) {
    validation.warnings.forEach(warn => {
        ElMessage({ type: 'warning', message: warn });
    });
}
```

## 安全改进总结

### 防止的攻击类型

1. **路径遍历攻击**
   - 阻止 `../` 序列
   - 使用路径规范化
   - 验证最终路径

2. **注入攻击**
   - 过滤危险字符：`<`, `>`, `|`, `*`, `?`, `\0`
   - 用户名白名单验证
   - JSON 格式严格验证

3. **弱密码风险**
   - 检测常见弱密码
   - 密码强度警告
   - 长度限制

4. **资源耗尽**
   - 路径长度限制（4096 字符）
   - 用户名长度限制（64 字符）
   - 密码长度限制（128 字符）
   - 用户数量限制（100 个）

5. **权限提升**
   - 端口特权警告
   - 文件权限严格验证
   - 路径目录类型验证

### 双重验证机制

- **前端验证**：提供即时反馈，改善用户体验
- **后端验证**：确保安全性，防止绕过前端验证

## 测试建议

### 单元测试用例

```rust
#[test]
fn test_path_traversal_attack() {
    assert!(sanitize_path("../../../etc/passwd").is_err());
    assert!(sanitize_path("/safe/path/../../../etc/passwd").is_err());
}

#[test]
fn test_sql_injection_in_username() {
    assert!(sanitize_username("admin'; DROP TABLE users;--").is_err());
}

#[test]
fn test_weak_password_detection() {
    let result = sanitize_password("123456");
    assert!(result.is_ok()); // 允许但警告
}
```

### 集成测试建议

1. 测试路径遍历攻击
2. 测试特殊字符注入
3. 测试边界值（长度限制）
4. 测试弱密码警告
5. 测试特权端口警告

## 后续改进建议

1. **密码哈希**：使用 bcrypt 或 argon2 替代明文存储
2. **日志记录**：记录所有验证失败事件
3. **速率限制**：防止暴力破解
4. **审计日志**：记录用户操作
5. **配置文件验证**：验证 tauri.conf.json 的 CSP 设置

## 文件变更列表

### 新增文件
- `src/utils/validation.ts` - 前端验证工具

### 修改文件
- `src-tauri/src/invoke_command.rs` - 后端验证逻辑
- `src/view/Ftp.vue` - 集成路径和端口验证
- `src/view/FtpAuth.vue` - 集成用户数据验证

## 版本信息

- 更新日期：2026-03-04
- 影响版本：0.1.1+
- 安全等级：高优先级
