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
use tauri::{
    menu::{Menu, MenuItem},
    Emitter, Manager,
};
use tauri_plugin_log::{Target, TargetKind};

pub mod ftp;
pub mod commands;
pub mod validators;

/// 全局应用状态
pub struct AppState {
    pub is_server_running: Arc<Mutex<bool>>,
}

/// 更新托盘菜单
pub fn update_tray_menu(app: &tauri::AppHandle, is_running: bool) -> Result<(), Box<dyn std::error::Error>> {
    // 创建托盘菜单项
    let show_item = MenuItem::with_id(app, "show", "显示主界面", true, None::<&str>)?;
    let toggle_text = if is_running { "停止 FTP" } else { "启动 FTP" };
    let toggle_item = MenuItem::with_id(app, "toggle", toggle_text, true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    // 创建托盘菜单
    let menu = Menu::with_items(app, &[&show_item, &toggle_item, &quit_item])?;

    // 获取托盘并设置新菜单
    if let Some(tray) = app.tray_by_id("main") {
        tray.set_menu(Some(menu))?;
    }

    Ok(())
}

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
            let app_state = Arc::new(Mutex::new(AppState {
                is_server_running: Arc::new(Mutex::new(false)),
            }));
            app.manage(app_state.clone());

            // 初始化托盘菜单（服务未运行）
            let app_handle = app.handle().clone();
            update_tray_menu(&app_handle, false)?;

            // 获取托盘并设置事件处理
            if let Some(tray) = app.tray_by_id("main") {
                tray.on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                                // 恢复任务栏/Dock 图标
                                #[cfg(target_os = "macos")]
                                {
                                    let _ = app.set_activation_policy(tauri::ActivationPolicy::Regular);
                                }
                                #[cfg(target_os = "windows")]
                                {
                                    let _ = window.set_skip_taskbar(false);
                                }
                            }
                        }
                        "toggle" => {
                            // 触发切换事件，由前端处理
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.emit("tray-toggle-ftp", ());
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
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
            commands::system::hide_dock_icon,
            commands::system::show_dock_icon,
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
        // 运行应用
        .run(tauri::generate_context!())
        .expect("error while running ftp Server application");
}
