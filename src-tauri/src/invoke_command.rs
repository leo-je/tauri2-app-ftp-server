use std::sync::{Arc, Mutex};

use crate::ftp::ftpworker::{FtpWorker, FtpWorkerConfig};

// 添加输入验证函数
pub fn validate_path(path: &str) -> bool {
    // 实现路径验证逻辑
    !path.is_empty()
}

pub fn validate_port(port: &str) -> bool {
    // 实现端口验证逻辑
    port.parse::<u16>().is_ok()
}

#[tauri::command]
pub fn start_ftp_server(
    state: tauri::State<'_, Arc<Mutex<FtpWorker>>>,
    path: String,
    port: String,
    users: String,
    is_anonymous: bool,
    fileauth: String,
) -> Result<String, String> {
    // 验证输入参数
    if !validate_path(&path) || !validate_port(&port) {
        return Err("无效的路径或端口".to_string());
    }

    let mut worker = match state.lock() {
        Ok(guard) => guard,
        Err(e) => {
            // error!("无法获取锁: {}", e);
            return Err(format!("无法获取锁: {}", e));
        }
    };

    if worker.is_running() {
        // info!("FTP 服务已启动");
        return Ok("服务已启动".to_string());
    }

    worker.set(FtpWorkerConfig {
        path,
        port,
        users,
        is_anonymous,
        fileauth,
    });

    // 启动 FTP 服务
    match worker.start() {
        Ok(_) => {
            // info!("FTP 服务启动成功");
            Ok("服务已启动".to_string())
        }
        Err(e) => {
            // error!("FTP 服务启动失败: {}", e);
            Err(format!("服务启动失败: {}", e))
        }
    }
}

#[tauri::command]
pub fn stop_ftp_server(state: tauri::State<'_, Arc<Mutex<FtpWorker>>>) -> Result<String, String> {
    let mut worker = match state.lock() {
        Ok(guard) => guard,
        Err(e) => {
            // error!("无法获取锁: {}", e);
            return Err(format!("无法获取锁: {}", e));
        }
    };

    match worker.stop() {
        Ok(_) => {
            // info!("FTP 服务已停止");
            Ok("服务已停止".to_string())
        }
        Err(e) => {
            // error!("FTP 服务停止失败: {}", e);
            Err(format!("FTP 服务停止失败: {}", e))
        }
    }
}
