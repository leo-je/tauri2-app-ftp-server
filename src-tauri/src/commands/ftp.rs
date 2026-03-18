//! FTP 服务器管理命令
//!
//! 包含启动和停止 FTP 服务器的 Tauri 命令。

use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::ftp::ftpworker::{FtpWorker, FtpWorkerConfig};
use crate::tray::AppState;
use crate::validators::{sanitize_file_auth, sanitize_path, sanitize_port, sanitize_users_json};

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
    let path_str = sanitized_path.to_str().ok_or("路径格式无效")?.to_string();

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
            if let Ok(mut state) = app_state.lock() {
                state.server_start_time = Some(Instant::now());
            }
            let _ = crate::tray::update_tray_menu(&app, true);
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
            if let Ok(mut state) = app_state.lock() {
                state.server_start_time = None;
            }
            let _ = crate::tray::update_tray_menu(&app, false);
            Ok("服务已停止".to_string())
        }
        Err(e) => Err(format!("FTP 服务停止失败: {}", e)),
    }
}
