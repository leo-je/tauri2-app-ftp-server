//! 输入验证和清理模块
//!
//! 该模块包含所有用于验证和清理用户输入的函数，包括：
//! - 路径清理和验证
//! - 端口清理和验证
//! - 用户名和密码清理
//! - 文件权限验证
//! - 用户列表 JSON 验证
//! - 权限检查

use std::path::PathBuf;

/// 允许的端口号范围
pub const MIN_PORT: u16 = 1;
/// FTP 标准端口
pub const FTP_STANDARD_PORT: u16 = 21;

/// 清理和验证路径
///
/// # 功能
/// - 去除首尾空白字符
/// - 规范化路径分隔符
/// - 解析相对路径和符号链接
/// - 验证路径是否存在且可访问
///
/// # 参数
/// * `path` - 待清理的路径字符串
///
/// # 返回值
/// * `Ok(PathBuf)` - 清理后的规范路径
/// * `Err(String)` - 错误信息
pub fn sanitize_path(path: &str) -> Result<PathBuf, String> {
    // 去除首尾空白
    let trimmed = path.trim();

    // 检查是否为空
    if trimmed.is_empty() {
        return Err("路径不能为空".to_string());
    }

    // 检查路径长度（防止过长路径攻击）
    if trimmed.len() > 4096 {
        return Err("路径长度超出限制".to_string());
    }

    // 检查危险字符和模式
    let dangerous_patterns = ["../", "..\\", "\0", "<", ">", "|", "*", "?"];
    for pattern in &dangerous_patterns {
        if trimmed.contains(pattern) {
            return Err(format!("路径包含非法字符或模式: {}", pattern));
        }
    }

    // 转换为 PathBuf
    let path_buf = PathBuf::from(trimmed);

    // 规范化路径（解析符号链接、相对路径等）
    let canonical_path = path_buf
        .canonicalize()
        .map_err(|e| format!("路径不存在或无法访问: {}", e))?;

    // 验证路径是否为目录
    if !canonical_path.is_dir() {
        return Err("指定路径不是目录".to_string());
    }

    Ok(canonical_path)
}

/// 验证路径是否有效（向后兼容的简化版本）
///
/// # 参数
/// * `path` - 待验证的路径字符串
///
/// # 返回值
/// * `true` - 路径有效（非空且存在）
/// * `false` - 路径无效
pub fn validate_path(path: &str) -> bool {
    sanitize_path(path).is_ok()
}

/// 清理和验证端口号
///
/// # 功能
/// - 去除首尾空白
/// - 验证是否为有效数字
/// - 检查端口范围
/// - 警告特权端口使用
///
/// # 参数
/// * `port` - 待清理的端口字符串
///
/// # 返回值
/// * `Ok(u16)` - 清理后的端口号
/// * `Err(String)` - 错误信息
pub fn sanitize_port(port: &str) -> Result<u16, String> {
    // 去除首尾空白
    let trimmed = port.trim();

    // 检查是否为空
    if trimmed.is_empty() {
        return Err("端口号不能为空".to_string());
    }

    // 解析端口号
    let port_num = trimmed
        .parse::<u16>()
        .map_err(|_| "端口号格式无效，请输入 1-65535 之间的数字".to_string())?;

    // 检查端口范围
    if port_num < MIN_PORT {
        return Err("端口号必须大于 0".to_string());
    }

    // 警告使用标准 FTP 端口（可能需要管理员权限）
    if port_num == FTP_STANDARD_PORT {
        eprintln!("警告: 使用标准 FTP 端口 21 可能需要管理员权限");
    }

    // 警告使用特权端口
    if port_num < 1024 {
        eprintln!("警告: 使用特权端口 {} 可能需要管理员权限", port_num);
    }

    Ok(port_num)
}

/// 验证端口号是否有效（向后兼容的简化版本）
///
/// # 参数
/// * `port` - 待验证的端口字符串
///
/// # 返回值
/// * `true` - 端口有效（可解析为 16 位无符号整数）
/// * `false` - 端口无效
pub fn validate_port(port: &str) -> bool {
    sanitize_port(port).is_ok()
}

/// 清理和验证用户名
///
/// # 功能
/// - 去除首尾空白
/// - 检查长度限制
/// - 过滤危险字符
///
/// # 参数
/// * `username` - 待清理的用户名
///
/// # 返回值
/// * `Ok(String)` - 清理后的用户名
/// * `Err(String)` - 错误信息
pub fn sanitize_username(username: &str) -> Result<String, String> {
    // 去除首尾空白
    let trimmed = username.trim();

    // 检查长度
    if trimmed.is_empty() {
        return Err("用户名不能为空".to_string());
    }
    if trimmed.len() > 64 {
        return Err("用户名长度不能超过 64 个字符".to_string());
    }

    // 检查非法字符（允许字母、数字、下划线、短横线）
    if !trimmed
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return Err("用户名只能包含字母、数字、下划线和短横线".to_string());
    }

    Ok(trimmed.to_string())
}

/// 清理和验证密码
///
/// # 功能
/// - 检查长度限制
/// - 记录弱密码警告（不强制要求）
///
/// # 参数
/// * `password` - 待清理的密码
///
/// # 返回值
/// * `Ok(String)` - 清理后的密码
/// * `Err(String)` - 错误信息
pub fn sanitize_password(password: &str) -> Result<String, String> {
    // 检查长度
    if password.is_empty() {
        return Err("密码不能为空".to_string());
    }
    if password.len() > 128 {
        return Err("密码长度不能超过 128 个字符".to_string());
    }

    // 检查密码强度（仅警告，不强制）
    if password.len() < 6 {
        eprintln!("警告: 密码长度小于 6 位，建议使用更强的密码");
    }

    // 检查常见弱密码
    let weak_passwords = ["123456", "password", "admin", "root", "12345678"];
    if weak_passwords.contains(&password) {
        eprintln!("警告: 检测到弱密码，建议使用更强的密码");
    }

    Ok(password.to_string())
}

/// 清理和验证文件权限标识
///
/// # 参数
/// * `file_auth` - 文件权限标识
///
/// # 返回值
/// * `Ok(String)` - 清理后的权限标识（"R" 或 "W"）
/// * `Err(String)` - 错误信息
pub fn sanitize_file_auth(file_auth: &str) -> Result<String, String> {
    let trimmed = file_auth.trim().to_uppercase();

    match trimmed.as_str() {
        "R" | "W" => Ok(trimmed),
        _ => Err("文件权限必须是 'R'（只读）或 'W'（读写）".to_string()),
    }
}

/// 清理和验证用户列表 JSON
///
/// # 功能
/// - 解析 JSON 格式
/// - 验证每个用户的字段
/// - 清理用户输入数据
///
/// # 参数
/// * `users_json` - 用户列表的 JSON 字符串
///
/// # 返回值
/// * `Ok(String)` - 清理后的用户列表 JSON
/// * `Err(String)` - 错误信息
pub fn sanitize_users_json(users_json: &str) -> Result<String, String> {
    // 如果为空，返回空数组
    if users_json.trim().is_empty() {
        return Ok("[]".to_string());
    }

    // 解析 JSON
    let users: Vec<serde_json::Value> = serde_json::from_str(users_json)
        .map_err(|e| format!("用户列表 JSON 格式无效: {}", e))?;

    // 验证用户数量限制
    if users.len() > 100 {
        return Err("用户数量不能超过 100".to_string());
    }

    // 清理和验证每个用户
    let mut sanitized_users = Vec::new();
    for (index, user) in users.iter().enumerate() {
        let username = user["username"]
            .as_str()
            .ok_or_else(|| format!("用户 {} 缺少用户名", index))?;
        let password = user["password"]
            .as_str()
            .ok_or_else(|| format!("用户 {} 缺少密码", index))?;
        let file_auth = user["fileAuth"]
            .as_str()
            .unwrap_or("R");

        // 清理各个字段
        let sanitized_username = sanitize_username(username)?;
        let sanitized_password = sanitize_password(password)?;
        let sanitized_file_auth = sanitize_file_auth(file_auth)?;

        sanitized_users.push(serde_json::json!({
            "username": sanitized_username,
            "password": sanitized_password,
            "fileAuth": sanitized_file_auth
        }));
    }

    // 转换回 JSON
    serde_json::to_string(&sanitized_users)
        .map_err(|e| format!("序列化用户数据失败: {}", e))
}

/// 检查写入权限
pub fn check_write_permission() -> bool {
    use std::fs;

    // 尝试在临时目录写入测试文件
    let temp_path = std::env::temp_dir().join("ftp_server_write_test.tmp");
    match fs::write(&temp_path, b"test") {
        Ok(_) => {
            let _ = fs::remove_file(&temp_path);
            true
        }
        Err(_) => false,
    }
}

/// 检查是否有权限绑定特权端口
pub fn check_privileged_port_permission() -> bool {
    // 尝试绑定一个特权端口来判断
    use std::net::TcpListener;

    // 使用 1023 作为测试端口
    match TcpListener::bind("127.0.0.1:1023") {
        Ok(listener) => {
            // 成功绑定，说明有权限
            drop(listener);
            true
        }
        Err(_) => false,
    }
}
