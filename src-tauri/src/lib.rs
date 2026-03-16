//! Tauri 应用程序主模块
//!
//! 这是 Tauri 应用的入口模块，负责：
//! - 初始化和配置 Tauri 应用
//! - 注册各种插件（日志、对话框、剪贴板等）
//! - 设置 FTP 工作线程状态管理
//! - 注册前端可调用的命令
//!
//! 了解更多关于 Tauri 命令：<https://tauri.app/develop/calling-rust/>

use std::sync::{Arc, Mutex};
use tauri::{Manager, RunEvent, WindowEvent};
use tauri_plugin_log::{Target, TargetKind};

pub mod ftp;
pub mod commands;
pub mod tray;
pub mod validators;

/// Tauri 应用入口函数
///
/// 构建并运行 Tauri 应用，配置所有必要的插件和处理器。
/// 该函数在桌面端和移动端（通过 mobile_entry_point）都会被调用。
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 初始化剪贴板插件
        .plugin(tauri_plugin_clipboard::init())
        // 初始化持久化存储插件
        .plugin(tauri_plugin_store::Builder::new().build())
        // 应用初始化设置
        .setup(move |app| {
            // 创建 FTP 工作线程并管理其状态
            let worker = Arc::new(Mutex::new(ftp::ftpworker::FtpWorker::new()));
            app.manage(worker);

            // 创建应用状态
            let app_state = Arc::new(Mutex::new(tray::AppState {
                is_server_running: Arc::new(Mutex::new(false)),
            }));
            app.manage(app_state.clone());

            // 初始化托盘菜单和事件处理
            let app_handle = app.handle().clone();
            tray::setup_tray_menu(&app_handle)?;

            // 拦截窗口关闭请求，改为隐藏窗口
            if let Some(window) = app.get_webview_window("main") {
                let app_handle_clone = app.handle().clone();
                window.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = tray::hide_main_window(app_handle_clone.clone());
                    }
                });
            }

            Ok(())
        })
        // 初始化操作系统信息插件
        .plugin(tauri_plugin_os::init())
        // 初始化增强版 Shell 插件（允许执行命令）
        .plugin(tauri_plugin_shellx::init(true))
        // 初始化对话框插件
        .plugin(tauri_plugin_dialog::init())
        // 初始化文件打开插件
        .plugin(tauri_plugin_opener::init())
        // 注册前端可调用的命令
        .invoke_handler(tauri::generate_handler![
            commands::ftp::start_ftp_server,
            commands::ftp::stop_ftp_server,
            commands::network::get_primary_ipv4,
            commands::system::get_system_info,
            commands::system::check_app_config,
            commands::system::check_permissions,
            commands::network::get_network_interfaces,
            commands::init::run_init_step,
            commands::init::get_init_status,
            commands::system::set_server_running,
            commands::system::get_server_running,
            tray::hide_main_window,
        ])
        // 初始化日志插件
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    // 输出到标准输出
                    Target::new(TargetKind::Stdout),
                    // 输出到日志目录
                    Target::new(TargetKind::LogDir { file_name: None }),
                    // 输出到 Webview 控制台
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        // 运行应用，处理 Dock 图标点击等事件
        .build(tauri::generate_context!())
        .expect("Failed to build Tauri application")
        .run(|app_handle, event| {
            if let RunEvent::Reopen { .. } = event {
                // macOS: 点击 Dock 图标时显示主窗口
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        });
}
