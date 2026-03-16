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

/// 托盘菜单翻译文本
#[derive(Clone, serde::Deserialize)]
pub struct TrayMenuText {
    pub show: String,
    pub start: String,
    pub stop: String,
    pub quit: String,
}

/// 全局托盘菜单翻译文本
static TRAY_MENU_TEXT: std::sync::LazyLock<Mutex<TrayMenuText>> = std::sync::LazyLock::new(|| {
    Mutex::new(TrayMenuText {
        show: "显示主界面".to_string(),
        start: "启动 FTP".to_string(),
        stop: "停止 FTP".to_string(),
        quit: "退出".to_string(),
    })
});

/// 更新托盘菜单文本
pub fn update_tray_menu_text(text: TrayMenuText) {
    if let Ok(mut menu_text) = TRAY_MENU_TEXT.lock() {
        *menu_text = text;
    }
}

/// 更新托盘菜单
pub fn update_tray_menu(
    app: &tauri::AppHandle,
    is_running: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let menu_text = TRAY_MENU_TEXT.lock().unwrap();

    let show_item = MenuItem::with_id(app, "show", &menu_text.show, true, None::<&str>)?;
    let toggle_text = if is_running {
        menu_text.stop.clone()
    } else {
        menu_text.start.clone()
    };
    let toggle_item = MenuItem::with_id(app, "toggle", &toggle_text, true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", &menu_text.quit, true, None::<&str>)?;

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
        tray.on_menu_event(|app, event| match event.id.as_ref() {
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

/// 更新托盘菜单语言
#[tauri::command]
pub fn update_tray_menu_language(
    app: tauri::AppHandle,
    is_running: bool,
    text: TrayMenuText,
) -> Result<(), String> {
    update_tray_menu_text(text);
    update_tray_menu(&app, is_running).map_err(|e| e.to_string())
}
