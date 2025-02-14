// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

pub mod ftpworker;
use crate::ftpworker::FtpWorker;

#[tauri::command]
fn start_ftp_server(
    state: tauri::State<'_, Arc<Mutex<FtpWorker>>>,
    path: String,
    port: String,
) -> Result<String, String> {
    // 检查是否已经有子线程在运行
    let mut worker = state.lock().unwrap();
    let is_start: bool = worker.is_running();
    if is_start {
        println!("Thread is already running");
        return Ok("服务已启动".to_string());
    }
    let p = path.clone();
    let p2 = port.clone();
    // let mut worker = worker.lock().unwrap();
    worker.set(p, p2);
    let _ = worker.start();
    // 等待子线程完成
    // handle.join().unwrap();
    println!("start_ftp_server-3");
    Ok(format!("服务已启动"))
}

#[tauri::command]
fn stop_ftp_server(state: tauri::State<'_, Arc<Mutex<FtpWorker>>>) -> Result<String, ()> {
    let mut worker = state.lock().unwrap();
    worker.stop();
    Ok(format!("服务已停止"))
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
        .expect("error while running tauri application");
}
