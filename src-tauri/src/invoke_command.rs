//! Tauri 命令调用模块
//!
//! 该模块包含前端可以调用的 Rust 函数，通过 Tauri 的 invoke 机制暴露给前端。
//! 主要功能包括：
//! - 启动/停止 FTP 服务器
//! - 获取本机 IPv4 地址列表
//! - 应用初始化和系统检查

use std::sync::{Arc, Mutex};

use crate::ftp::ftpworker::{FtpWorker, FtpWorkerConfig};
use crate::AppState;
use get_if_addrs::get_if_addrs;
use std::net::IpAddr;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tauri_plugin_os::{arch, platform, version};

/// 允许的端口号范围
const MIN_PORT: u16 = 1;
/// FTP 标准端口
const FTP_STANDARD_PORT: u16 = 21;

/// 初始化步骤枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InitStep {
    SystemCheck,
    ConfigLoad,
    ServiceInit,
    Ready,
}

/// 初始化步骤请求
#[derive(Debug, Deserialize)]
pub struct InitStepRequest {
    pub step: String,
}

/// 初始化步骤响应
#[derive(Debug, Serialize)]
pub struct InitStepResponse {
    pub success: bool,
    pub step: String,
    pub progress: u8,
    pub message: String,
    pub can_continue: bool,
    pub estimated_time_ms: Option<u32>,
}

/// 初始化状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitStatus {
    pub step: String,
    pub progress: u8,
    pub message: String,
    pub status: String, // "pending", "running", "completed", "error"
}

/// 系统信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub platform: String,
    pub arch: String,
    pub version: String,
    pub hostname: String,
}

/// 配置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub config_exists: bool,
    pub config_valid: bool,
    pub default_path: Option<String>,
    pub default_port: u16,
}

/// 权限检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionCheck {
    pub has_write_permission: bool,
    pub has_network_permission: bool,
    pub can_bind_privileged_port: bool,
}

/// 初始化检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitCheckResult {
    pub success: bool,
    pub step: String,
    pub progress: u8,
    pub message: String,
    pub message_key: String,
    pub message_params: Vec<String>,
    pub can_continue: bool,
    pub estimated_time_ms: Option<u32>,
}

/// 获取系统信息
///
/// 检测当前操作系统、架构和版本信息
#[tauri::command]
pub fn get_system_info() -> Result<SystemInfo, String> {
    let platform_str = platform().to_string();
    let arch_str = arch().to_string();
    let version_str = version().to_string();

    // 获取主机名 - 使用环境变量
    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .or_else(|_| std::env::var("NAME"))
        .unwrap_or_else(|_| "unknown".to_string());

    Ok(SystemInfo {
        platform: platform_str,
        arch: arch_str,
        version: version_str,
        hostname,
    })
}

/// 检查应用配置
///
/// 检查配置文件是否存在且有效
#[tauri::command]
pub fn check_app_config(app_handle: tauri::AppHandle) -> Result<AppConfig, String> {
    use tauri_plugin_store::StoreExt;

    // 尝试获取存储
    let store = app_handle.store("app-config.json")
        .map_err(|e| format!("无法访问配置存储: {}", e))?;

    // 检查是否有默认配置
    let default_path = store.get("defaultPath")
        .and_then(|v| v.as_str().map(|s| s.to_string()));

    let default_port = store.get("defaultPort")
        .and_then(|v| v.as_u64())
        .map(|p| p as u16)
        .unwrap_or(2121);

    let config_exists = default_path.is_some();

    Ok(AppConfig {
        config_exists,
        config_valid: config_exists,
        default_path,
        default_port,
    })
}

/// 检查权限状态
///
/// 检查应用运行所需的各项权限
#[tauri::command]
pub fn check_permissions() -> Result<PermissionCheck, String> {
    let has_write_permission = check_write_permission();
    let has_network_permission = true; // Tauri 应用中网络权限通常是默认的
    let can_bind_privileged_port = check_privileged_port_permission();

    Ok(PermissionCheck {
        has_write_permission,
        has_network_permission,
        can_bind_privileged_port,
    })
}

/// 检查写入权限
fn check_write_permission() -> bool {
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
fn check_privileged_port_permission() -> bool {
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

/// 获取网络接口信息
#[tauri::command]
pub fn get_network_interfaces() -> Result<Vec<String>, String> {
    let mut interfaces = vec![];

    if let Ok(ifaces) = get_if_addrs() {
        for iface in ifaces {
            if let IpAddr::V4(ipv4) = iface.ip() {
                if !ipv4.is_loopback() {
                    interfaces.push(format!("{}: {}", iface.name, ipv4));
                }
            }
        }
    }

    Ok(interfaces)
}

/// 执行初始化检查步骤
///
/// 根据步骤名称执行相应的初始化检查
#[tauri::command]
pub async fn run_init_step(
    step: String,
    app_handle: tauri::AppHandle,
) -> Result<InitCheckResult, String> {
    match step.as_str() {
        "system_check" => {
            // 随机睡眠 500-800ms
            let sleep_ms = rand::random::<u64>() % 301 + 500;
            tokio::time::sleep(tokio::time::Duration::from_millis(sleep_ms)).await;

            let sys_info = get_system_info()?;

            Ok(InitCheckResult {
                success: true,
                step: "system_check".to_string(),
                progress: 25,
                message: format!("{} {}", sys_info.platform, sys_info.arch),
                message_key: "splash.messages.systemCheckComplete".to_string(),
                message_params: vec![sys_info.platform, sys_info.arch],
                can_continue: true,
                estimated_time_ms: None,
            })
        }
        "config_load" => {
            // 随机睡眠 300-500ms
            let sleep_ms = rand::random::<u64>() % 201 + 300;
            tokio::time::sleep(tokio::time::Duration::from_millis(sleep_ms)).await;

            let config = check_app_config(app_handle)?;

            let (message_key, message) = if config.config_exists {
                ("splash.messages.configLoaded", "Config loaded".to_string())
            } else {
                ("splash.messages.usingDefaultConfig", "Using default config".to_string())
            };

            Ok(InitCheckResult {
                success: true,
                step: "config_load".to_string(),
                progress: 50,
                message,
                message_key: message_key.to_string(),
                message_params: vec![],
                can_continue: true,
                estimated_time_ms: None,
            })
        }
        "service_init" => {
            // 随机睡眠 800-1200ms
            let sleep_ms = rand::random::<u64>() % 401 + 800;
            tokio::time::sleep(tokio::time::Duration::from_millis(sleep_ms)).await;

            // 检查网络接口
            let interfaces = get_network_interfaces()?;
            let interface_count = interfaces.len().to_string();

            Ok(InitCheckResult {
                success: true,
                step: "service_init".to_string(),
                progress: 75,
                message: format!("Found {} interfaces", interface_count),
                message_key: "splash.messages.interfacesFound".to_string(),
                message_params: vec![interface_count],
                can_continue: true,
                estimated_time_ms: None,
            })
        }
        "ready" => {
            // 随机睡眠 200-300ms
            let sleep_ms = rand::random::<u64>() % 101 + 200;
            tokio::time::sleep(tokio::time::Duration::from_millis(sleep_ms)).await;

            Ok(InitCheckResult {
                success: true,
                step: "ready".to_string(),
                progress: 100,
                message: "System ready".to_string(),
                message_key: "splash.messages.systemReady".to_string(),
                message_params: vec![],
                can_continue: true,
                estimated_time_ms: None,
            })
        }
        _ => Err(format!("未知的初始化步骤: {}", step)),
    }
}

/// 获取完整的初始化状态
#[tauri::command]
pub fn get_init_status() -> Vec<InitStatus> {
    vec![
        InitStatus {
            step: "system_check".to_string(),
            progress: 0,
            message: "等待系统检测".to_string(),
            status: "pending".to_string(),
        },
        InitStatus {
            step: "config_load".to_string(),
            progress: 0,
            message: "等待配置加载".to_string(),
            status: "pending".to_string(),
        },
        InitStatus {
            step: "service_init".to_string(),
            progress: 0,
            message: "等待服务初始化".to_string(),
            status: "pending".to_string(),
        },
        InitStatus {
            step: "ready".to_string(),
            progress: 0,
            message: "等待准备就绪".to_string(),
            status: "pending".to_string(),
        },
    ]
}
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

/// 启动 FTP 服务器（Tauri 命令）
///
/// # 参数
/// * `app` - Tauri 应用句柄，用于更新托盘菜单
/// * `app_state` - 应用状态，用于更新运行状态
/// * `state` - Tauri 应用状态，包含 FtpWorker 实例
/// * `path` - FTP 根目录路径
/// * `port` - FTP 服务监听端口
/// * `users` - 用户列表（JSON 格式字符串）
/// * `is_anonymous` - 是否允许匿名访问
/// * `file_auth` - 默认文件权限
///
/// # 返回值
/// * `Ok(String)` - 启动成功，返回 "服务已启动"
/// * `Err(String)` - 启动失败，返回错误信息
///
/// # 安全性
/// 所有输入参数都会经过严格的清理和验证：
/// - 路径：规范化、检查存在性、防止路径遍历攻击
/// - 端口：验证范围、警告特权端口
/// - 用户数据：清理用户名、密码、权限标识
///
/// # 示例
/// 前端调用方式：
/// ```javascript
/// await invoke('start_ftp_server', {
///   path: '/home/ftp',
///   port: '2121',
///   users: '[{"username":"admin","password":"123"}]',
///   is_anonymous: false,
///   file_auth: 'W'
/// });
/// ```
#[tauri::command]
pub fn start_ftp_server(
    app: tauri::AppHandle,
    app_state: tauri::State<'_, Arc<Mutex<AppState>>>,
    state: tauri::State<'_, Arc<Mutex<FtpWorker>>>,
    path: String,
    port: String,
    users: String,
    is_anonymous: bool,
    file_auth: String,
) -> Result<String, String> {
    // 清理和验证路径
    let sanitized_path = sanitize_path(&path)?;
    let path_str = sanitized_path
        .to_str()
        .ok_or("路径格式无效")?
        .to_string();

    // 清理和验证端口
    let sanitized_port = sanitize_port(&port)?;
    let port_str = sanitized_port.to_string();

    // 清理和验证用户列表
    let sanitized_users = sanitize_users_json(&users)?;

    // 清理和验证文件权限
    let sanitized_file_auth = sanitize_file_auth(&file_auth)?;

    // 获取锁，如果锁中毒则尝试恢复
    let mut worker = match state.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            // 锁中毒时，记录警告并使用毒锁中的数据
            eprintln!("警告: Mutex 锁中毒，尝试恢复: {}", poisoned);
            poisoned.into_inner()
        }
    };

    // 检查服务是否已在运行
    if worker.is_running() {
        return Ok("服务已启动".to_string());
    }

    // 设置配置并启动服务
    worker.set(FtpWorkerConfig {
        path: path_str,
        port: port_str,
        users: sanitized_users,
        is_anonymous,
        file_auth: sanitized_file_auth,
        ..Default::default()
    });

    match worker.start() {
        Ok(_) => {
            // 更新托盘菜单和运行状态
            if let Ok(state) = app_state.lock() {
                if let Ok(mut is_running) = state.is_server_running.lock() {
                    *is_running = true;
                }
            }
            let _ = crate::update_tray_menu(&app, true);
            Ok("服务已启动".to_string())
        }
        Err(e) => Err(format!("服务启动失败: {}", e)),
    }
}

/// 停止 FTP 服务器（Tauri 命令）
///
/// # 参数
/// * `app` - Tauri 应用句柄，用于更新托盘菜单
/// * `app_state` - 应用状态，用于更新运行状态
/// * `state` - Tauri 应用状态，包含 FtpWorker 实例
///
/// # 返回值
/// * `Ok(String)` - 停止成功，返回 "服务已停止"
/// * `Err(String)` - 停止失败，返回错误信息
#[tauri::command]
pub fn stop_ftp_server(
    app: tauri::AppHandle,
    app_state: tauri::State<'_, Arc<Mutex<AppState>>>,
    state: tauri::State<'_, Arc<Mutex<FtpWorker>>>,
) -> Result<String, String> {
    // 获取锁，如果锁中毒则尝试恢复
    let mut worker = match state.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            // 锁中毒时，记录警告并使用毒锁中的数据
            eprintln!("警告: Mutex 锁中毒，尝试恢复: {}", poisoned);
            poisoned.into_inner()
        }
    };

    match worker.stop() {
        Ok(_) => {
            // 更新托盘菜单和运行状态
            if let Ok(state) = app_state.lock() {
                if let Ok(mut is_running) = state.is_server_running.lock() {
                    *is_running = false;
                }
            }
            let _ = crate::update_tray_menu(&app, false);
            Ok("服务已停止".to_string())
        }
        Err(e) => Err(format!("FTP 服务停止失败: {}", e)),
    }
}

/// 获取本机主 IPv4 地址列表（Tauri 命令）
///
/// 遍历所有网络接口，返回非回环的 IPv4 地址列表。
/// 用于前端显示可供连接的 FTP 服务器地址。
///
/// # 返回值
/// 返回 IPv4 地址字符串数组，例如：["192.168.1.100", "10.0.0.5"]
#[tauri::command]
pub fn get_primary_ipv4() -> Vec<String> {
    let mut result = vec![];

    if let Ok(ifaces) = get_if_addrs() {
        for iface in ifaces {
            if let IpAddr::V4(ipv4) = iface.ip() {
                // 过滤掉回环地址（127.0.0.1）
                if !ipv4.is_loopback() {
                    result.push(ipv4.to_string());
                }
            }
        }
    }

    result
}

/// 设置服务运行状态
#[tauri::command]
pub fn set_server_running(
    app: tauri::AppHandle,
    app_state: tauri::State<'_, Arc<Mutex<AppState>>>,
    running: bool,
) -> Result<(), String> {
    // 更新状态
    if let Ok(state) = app_state.lock() {
        if let Ok(mut is_running) = state.is_server_running.lock() {
            *is_running = running;
        }
    }

    // 更新托盘菜单
    crate::update_tray_menu(&app, running)
        .map_err(|e| format!("更新托盘菜单失败: {}", e))?;

    Ok(())
}

/// 获取服务运行状态
#[tauri::command]
pub fn get_server_running(
    app_state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<bool, String> {
    if let Ok(state) = app_state.lock() {
        if let Ok(is_running) = state.is_server_running.lock() {
            return Ok(*is_running);
        }
    }
    Ok(false)
}
