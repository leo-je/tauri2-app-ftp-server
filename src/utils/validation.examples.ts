/**
 * 输入验证测试示例
 *
 * 演示如何使用验证工具进行输入清理
 */

import {
  validatePath,
  validatePort,
  validateUsername,
  validatePassword,
  validateFileAuth,
  validateUser,
  validateUsers
} from './validation';

// ============================================
// 路径验证示例
// ============================================

console.log('=== 路径验证示例 ===');

// 有效路径
console.log(validatePath('/home/user/ftp'));
// { valid: true }

// 路径遍历攻击
console.log(validatePath('../../../etc/passwd'));
// { valid: false, error: '路径包含非法字符或模式: ../' }

// 空字节注入
console.log(validatePath('/safe/path\0/../../etc/passwd'));
// { valid: false, error: '路径包含非法字符或模式: \0' }

// Shell 注入
console.log(validatePath('/path | rm -rf /'));
// { valid: false, error: '路径包含非法字符或模式: |' }

// ============================================
// 端口验证示例
// ============================================

console.log('\n=== 端口验证示例 ===');

// 有效端口
console.log(validatePort(2121));
// { valid: true }

// 标准端口（警告）
console.log(validatePort(21));
// { valid: true, warning: '使用标准 FTP 端口 21 可能需要管理员权限' }

// 特权端口（警告）
console.log(validatePort(80));
// { valid: true, warning: '使用特权端口 80 可能需要管理员权限' }

// 无效端口
console.log(validatePort(0));
// { valid: false, error: '端口号必须在 1-65535 之间' }

console.log(validatePort(70000));
// { valid: false, error: '端口号必须在 1-65535 之间' }

// ============================================
// 用户名验证示例
// ============================================

console.log('\n=== 用户名验证示例 ===');

// 有效用户名
console.log(validateUsername('admin_123'));
// { valid: true }

console.log(validateUsername('user-name'));
// { valid: true }

// 包含非法字符
console.log(validateUsername('admin;drop'));
// { valid: false, error: '用户名只能包含字母、数字、下划线和短横线' }

console.log(validateUsername('user@domain'));
// { valid: false, error: '用户名只能包含字母、数字、下划线和短横线' }

// 过长用户名
console.log(validateUsername('a'.repeat(65)));
// { valid: false, error: '用户名长度不能超过 64 个字符' }

// ============================================
// 密码验证示例
// ============================================

console.log('\n=== 密码验证示例 ===');

// 强密码
console.log(validatePassword('StrongP@ss123!'));
// { valid: true }

// 弱密码（警告）
console.log(validatePassword('123'));
// { valid: true, warning: '密码长度小于 6 位，建议使用更强的密码' }

// 常见弱密码
console.log(validatePassword('123456'));
// { valid: true, warning: '检测到弱密码，建议使用更强的密码' }

console.log(validatePassword('password'));
// { valid: true, warning: '检测到弱密码，建议使用更强的密码' }

// 过长密码
console.log(validatePassword('a'.repeat(129)));
// { valid: false, error: '密码长度不能超过 128 个字符' }

// ============================================
// 文件权限验证示例
// ============================================

console.log('\n=== 文件权限验证示例 ===');

// 有效权限
console.log(validateFileAuth('R'));
// { valid: true }

console.log(validateFileAuth('w'));
// { valid: true } (自动转大写)

// 无效权限
console.log(validateFileAuth('X'));
// { valid: false, error: "文件权限必须是 'R'（只读）或 'W'（读写）" }

// ============================================
// 单个用户验证示例
// ============================================

console.log('\n=== 单个用户验证示例 ===');

// 有效用户
console.log(validateUser({
  username: 'admin',
  password: 'SecurePass123',
  fileAuth: 'W'
}));
// { valid: true, errors: [], warnings: [] }

// 有问题的用户
console.log(validateUser({
  username: 'admin;drop',
  password: '123456',
  fileAuth: 'X'
}));
// {
//   valid: false,
//   errors: [
//     '用户名只能包含字母、数字、下划线和短横线',
//     "文件权限必须是 'R'（只读）或 'W'（读写）"
//   ],
//   warnings: ['检测到弱密码，建议使用更强的密码']
// }

// ============================================
// 用户列表验证示例
// ============================================

console.log('\n=== 用户列表验证示例 ===');

// 有效用户列表
console.log(validateUsers([
  { username: 'admin', password: 'AdminPass123', fileAuth: 'W' },
  { username: 'user1', password: 'UserPass123', fileAuth: 'R' }
]));
// { valid: true, errors: [], warnings: [] }

// 混合用户列表
console.log(validateUsers([
  { username: 'admin', password: '123456', fileAuth: 'W' },
  { username: 'user;drop', password: 'UserPass123', fileAuth: 'X' }
]));
// {
//   valid: false,
//   errors: [
//     '用户 2: 用户名只能包含字母、数字、下划线和短横线',
//     '用户 2: 文件权限必须是 \'R\'（只读）或 \'W\'（读写）'
//   ],
//   warnings: [
//     '用户 1: 检测到弱密码，建议使用更强的密码'
//   ]
// }

// ============================================
// 在 Vue 组件中的使用示例
// ============================================

/**
 * 在 Vue 组件中使用验证
 */
export function exampleUsageInVue() {
  // 示例 1: 启动服务器前验证
  function validateBeforeStart(path: string, port: number) {
    // 验证路径
    const pathResult = validatePath(path);
    if (!pathResult.valid) {
      console.error('路径错误:', pathResult.error);
      return false;
    }

    // 验证端口
    const portResult = validatePort(port);
    if (!portResult.valid) {
      console.error('端口错误:', portResult.error);
      return false;
    }

    // 显示警告但允许继续
    if (portResult.warning) {
      console.warn('端口警告:', portResult.warning);
    }

    return true;
  }

  // 示例 2: 保存用户前验证
  function validateBeforeSave(user: { username: string; password: string; fileAuth?: string }) {
    const result = validateUser(user);

    // 显示所有错误
    if (result.errors.length > 0) {
      result.errors.forEach(err => console.error('错误:', err));
      return false;
    }

    // 显示警告但允许继续
    if (result.warnings.length > 0) {
      result.warnings.forEach(warn => console.warn('警告:', warn));
    }

    return true;
  }

  // 示例 3: 批量验证用户
  function validateUserList(users: Array<{ username: string; password: string; fileAuth?: string }>) {
    const result = validateUsers(users);

    if (!result.valid) {
      console.error('发现错误:', result.errors);
      return false;
    }

    if (result.warnings.length > 0) {
      console.warn('警告:', result.warnings);
    }

    return true;
  }

  return {
    validateBeforeStart,
    validateBeforeSave,
    validateUserList
  };
}

// ============================================
// 攻击防护演示
// ============================================

console.log('\n=== 攻击防护演示 ===');

// SQL 注入尝试
console.log('SQL 注入尝试:');
console.log(validateUsername("admin'; DROP TABLE users;--"));
// { valid: false, error: '用户名只能包含字母、数字、下划线和短横线' }

// 路径遍历尝试
console.log('\n路径遍历尝试:');
console.log(validatePath('/var/ftp/../../../etc/shadow'));
// { valid: false, error: '路径包含非法字符或模式: ../' }

// XSS 尝试
console.log('\nXSS 尝试:');
console.log(validateUsername('<script>alert("XSS")</script>'));
// { valid: false, error: '用户名只能包含字母、数字、下划线和短横线' }

// 命令注入尝试
console.log('\n命令注入尝试:');
console.log(validatePath('/var/ftp; rm -rf /'));
// { valid: false, error: '路径包含非法字符或模式: ;' }

export {};
