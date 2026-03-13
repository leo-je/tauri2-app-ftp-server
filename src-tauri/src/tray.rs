//! 托盘图标和菜单管理模块
//!
//! 负责管理应用托盘图标、菜单项和 Dock/任务栏图标的显示/隐藏。
//! 包括：
//! - 托盘菜单创建和更新
//! - 托盘菜单事件处理
//! - Dock 图标 / 任务栏图标的显示和隐藏

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
    // 初始化托盘菜单（服务未运行）
    update_tray_menu(app, false)?;

    // 设置托盘菜单事件处理
    if let Some(tray) = app.tray_by_id("main") {
        tray.on_menu_event(|app, event| {
            match event.id.as_ref() {
                "show" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
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

/// 隐藏 Dock 图标 / 任务栏图标
///
/// macOS: 将应用激活策略设为 Accessory，隐藏 Dock 栏图标
/// Windows: 从任务栏移除窗口，仅保留托盘图标
#[tauri::command]
pub fn hide_dock_icon(app: tauri::AppHandle) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        app.set_activation_policy(tauri::ActivationPolicy::Accessory)
            .map_err(|e| format!("隐藏 Dock 图标失败: {}", e))?;
    }
    #[cfg(target_os = "windows")]
    {
        if let Some(window) = app.get_webview_window("main") {
            window
                .set_skip_taskbar(true)
                .map_err(|e| format!("隐藏任务栏图标失败: {}", e))?;
        }
    }
    Ok(())
}

/// 显示 Dock 图标 / 任务栏图标
///
/// macOS: 将应用激活策略恢复为 Regular，重新在 Dock 栏显示图标
/// Windows: 将窗口重新添加到任务栏
#[tauri::command]
pub fn show_dock_icon(app: tauri::AppHandle) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        app.set_activation_policy(tauri::ActivationPolicy::Regular)
            .map_err(|e| format!("显示 Dock 图标失败: {}", e))?;
    }
    #[cfg(target_os = "windows")]
    {
        if let Some(window) = app.get_webview_window("main") {
            window
                .set_skip_taskbar(false)
                .map_err(|e| format!("显示任务栏图标失败: {}", e))?;
        }
    }
    Ok(())
}
