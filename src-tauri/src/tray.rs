//! 托盘图标和菜单管理模块
//!
//! 负责管理应用托盘图标和菜单项。
//! 包括：
//! - 托盘菜单创建和更新
//! - 托盘菜单事件处理

use std::sync::{Arc, Mutex};
use tauri::{
    menu::{Menu, MenuItem},
    Emitter, Manager,
};

/// 全局应用状态
pub struct AppState {
    pub is_server_running: Arc<Mutex<bool>>,
}

/// 更新托盘菜单
pub fn update_tray_menu(app: &tauri::AppHandle, is_running: bool) -> Result<(), Box<dyn std::error::Error>> {
    let show_item = MenuItem::with_id(app, "show", "显示主界面", true, None::<&str>)?;
    let toggle_text = if is_running { "停止 FTP" } else { "启动 FTP" };
    let toggle_item = MenuItem::with_id(app, "toggle", toggle_text, true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show_item, &toggle_item, &quit_item])?;

    if let Some(tray) = app.tray_by_id("main") {
        tray.set_menu(Some(menu))?;
    }

    Ok(())
}

/// 初始化托盘菜单和事件处理
pub fn setup_tray_menu(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    update_tray_menu(app, false)?;

    if let Some(tray) = app.tray_by_id("main") {
        tray.on_menu_event(|app, event| {
            match event.id.as_ref() {
                "show" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "toggle" => {
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
}

/// 隐藏主窗口
#[tauri::command]
pub fn hide_main_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|e| format!("隐藏窗口失败: {}", e))?;
    }
    Ok(())
}
