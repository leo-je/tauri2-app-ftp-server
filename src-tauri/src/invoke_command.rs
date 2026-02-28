//! Tauri 命令调用模块
//!
//! 该模块包含前端可以调用的 Rust 函数，通过 Tauri 的 invoke 机制暴露给前端。
//! 主要功能包括：
//! - 启动/停止 FTP 服务器
//! - 获取本机 IPv4 地址列表

use std::sync::{Arc, Mutex};

use crate::ftp::ftpworker::{FtpWorker, FtpWorkerConfig};
use get_if_addrs::get_if_addrs;
use std::net::IpAddr;
use std::path::Path;

/// 验证路径是否有效
///
/// # 参数
/// * `path` - 待验证的路径字符串
///
/// # 返回值
/// * `true` - 路径有效（非空且存在）
/// * `false` - 路径无效
pub fn validate_path(path: &str) -> bool {
    !path.is_empty() && Path::new(path).exists()
}

/// 验证端口号是否有效
///
/// # 参数
/// * `port` - 待验证的端口字符串
///
/// # 返回值
/// * `true` - 端口有效（可解析为 16 位无符号整数）
/// * `false` - 端口无效
pub fn validate_port(port: &str) -> bool {
    port.parse::<u16>().is_ok()
}

/// 启动 FTP 服务器（Tauri 命令）
///
/// # 参数
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
    state: tauri::State<'_, Arc<Mutex<FtpWorker>>>,
    path: String,
    port: String,
    users: String,
    is_anonymous: bool,
    file_auth: String,
) -> Result<String, String> {
    // 验证输入参数
    if !validate_path(&path) {
        return Err("路径无效或不存在".to_string());
    }
    if !validate_port(&port) {
        return Err("端口号无效".to_string());
    }

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
        path,
        port,
        users,
        is_anonymous,
        file_auth,
        ..Default::default()
    });

    match worker.start() {
        Ok(_) => Ok("服务已启动".to_string()),
        Err(e) => Err(format!("服务启动失败: {}", e)),
    }
}

/// 停止 FTP 服务器（Tauri 命令）
///
/// # 参数
/// * `state` - Tauri 应用状态，包含 FtpWorker 实例
///
/// # 返回值
/// * `Ok(String)` - 停止成功，返回 "服务已停止"
/// * `Err(String)` - 停止失败，返回错误信息
#[tauri::command]
pub fn stop_ftp_server(state: tauri::State<'_, Arc<Mutex<FtpWorker>>>) -> Result<String, String> {
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
        Ok(_) => Ok("服务已停止".to_string()),
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
