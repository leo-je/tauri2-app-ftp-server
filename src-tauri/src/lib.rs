// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

pub mod ftpworker;
use crate::ftpworker::FtpWorker;


// 添加输入验证函数
fn validate_path(path: &str) -> bool {
    // 实现路径验证逻辑
    !path.is_empty()
}

fn validate_port(port: &str) -> bool {
    // 实现端口验证逻辑
    port.parse::<u16>().is_ok()
}

#[tauri::command]
fn start_ftp_server(
    state: tauri::State<'_, Arc<Mutex<FtpWorker>>>,
    path: String,
    port: String,
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

    // 设置路径和端口
    worker.set(path.clone(), port.clone());

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
fn stop_ftp_server(state: tauri::State<'_, Arc<Mutex<FtpWorker>>>) -> Result<String, String> {
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
        },
        Err(e) => {
            // error!("FTP 服务停止失败: {}", e);
            Err(format!("FTP 服务停止失败: {}", e))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(move |app| {
            let worker = Arc::new(Mutex::new(FtpWorker::new()));
            app.manage(worker);
            Ok(())
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shellx::init(true))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_ftp_server, stop_ftp_server,])
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running ftp Server application");
}
