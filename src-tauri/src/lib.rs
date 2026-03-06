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
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

pub mod ftp;
pub mod invoke_command;

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
            invoke_command::start_ftp_server,
            invoke_command::stop_ftp_server,
            invoke_command::get_primary_ipv4,
            invoke_command::get_system_info,
            invoke_command::check_app_config,
            invoke_command::check_permissions,
            invoke_command::get_network_interfaces,
            invoke_command::run_init_step,
            invoke_command::get_init_status,
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
