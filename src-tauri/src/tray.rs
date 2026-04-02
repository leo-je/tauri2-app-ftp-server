//! 托盘图标和菜单管理模块
//!
//! 负责管理应用托盘图标和菜单项。
//! 包括：
//! - 托盘菜单创建和更新
//! - 托盘菜单事件处理

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    Emitter, Manager, Wry,
};

/// 全局应用状态
pub struct AppState {
    pub is_server_running: Arc<Mutex<bool>>,
    pub server_start_time: Option<Instant>,
}

/// 托盘菜单翻译文本
#[derive(Clone, serde::Deserialize)]
pub struct TrayMenuText {
    pub show: String,
    pub start: String,
    pub stop: String,
    pub quit: String,
    #[serde(default = "default_uptime_label")]
    pub uptime: String,
}

fn default_uptime_label() -> String {
    "运行时间".to_string()
}

/// 全局托盘菜单翻译文本
static TRAY_MENU_TEXT: std::sync::LazyLock<Mutex<TrayMenuText>> = std::sync::LazyLock::new(|| {
    Mutex::new(TrayMenuText {
        show: "显示主界面".to_string(),
        start: "启动 FTP".to_string(),
        stop: "停止 FTP".to_string(),
        quit: "退出".to_string(),
        uptime: "运行时间".to_string(),
    })
});

/// 全局菜单项引用（用于文本更新，避免重建菜单）
struct TrayMenuItems {
    show: MenuItem<Wry>,
    toggle: MenuItem<Wry>,
    quit: MenuItem<Wry>,
    uptime: MenuItem<Wry>,
}

static TRAY_MENU_ITEMS: std::sync::LazyLock<Mutex<Option<TrayMenuItems>>> =
    std::sync::LazyLock::new(|| Mutex::new(None));

fn format_uptime(server_start_time: Option<Instant>, label: &str) -> String {
    match server_start_time {
        Some(start) => {
            let elapsed = start.elapsed();
            let hours = elapsed.as_secs() / 3600;
            let minutes = (elapsed.as_secs() % 3600) / 60;
            let seconds = elapsed.as_secs() % 60;
            format!("{}: {:02}:{:02}:{:02}", label, hours, minutes, seconds)
        }
        None => format!("{}: --:--:--", label),
    }
}

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

    let uptime_display = {
        let state = app.state::<Arc<Mutex<AppState>>>();
        let state = state.lock().unwrap();
        format_uptime(state.server_start_time, &menu_text.uptime)
    };

    let uptime_item = MenuItem::with_id(app, "uptime", &uptime_display, false, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let show_item = MenuItem::with_id(app, "show", &menu_text.show, true, None::<&str>)?;
    let toggle_text = if is_running {
        menu_text.stop.clone()
    } else {
        menu_text.start.clone()
    };
    let toggle_item = MenuItem::with_id(app, "toggle", &toggle_text, true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", &menu_text.quit, true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &uptime_item,
            &separator,
            &show_item,
            &toggle_item,
            &quit_item,
        ],
    )?;

    if let Some(tray) = app.tray_by_id("main") {
        tray.set_menu(Some(menu))?;
    }

    // 存储所有菜单项引用，供后续文本更新（避免重建菜单）
    if let Ok(mut items) = TRAY_MENU_ITEMS.lock() {
        *items = Some(TrayMenuItems {
            show: show_item,
            toggle: toggle_item,
            quit: quit_item,
            uptime: uptime_item,
        });
    }

    Ok(())
}

/// 初始化托盘菜单和事件处理
pub fn setup_tray_menu(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    update_tray_menu(app, false)?;

    if let Some(tray) = app.tray_by_id("main") {
        tray.on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                #[cfg(target_os = "macos")]
                let _ = app.set_activation_policy(tauri::ActivationPolicy::Regular);
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

    let app_handle = app.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));

        let uptime_display = {
            let state = app_handle.state::<Arc<Mutex<AppState>>>();
            let app_state = Arc::clone(&*state);
            let menu_text = match TRAY_MENU_TEXT.lock() {
                Ok(t) => t,
                Err(_) => continue,
            };
            let result = match app_state.lock() {
                Ok(s) => format_uptime(s.server_start_time, &menu_text.uptime),
                Err(_) => continue,
            };
            result
        };

        // 只更新运行时间菜单项的文字，不重建菜单（避免关闭已打开的菜单）
        if let Ok(items_guard) = TRAY_MENU_ITEMS.lock() {
            if let Some(ref items) = *items_guard {
                let _ = items.uptime.set_text(&uptime_display);
            }
        }
    });

    Ok(())
}

/// 隐藏主窗口
#[tauri::command]
pub fn hide_main_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let is_visible = window.is_visible().unwrap_or(false);
        if is_visible {
            window.hide().map_err(|e| format!("隐藏窗口失败: {}", e))?;
        }
    }
    // macOS: 隐藏 dock 栏图标
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory)
        .map_err(|e| format!("隐藏 dock 图标失败: {}", e))?;
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

    // 只更新现有菜单项的文本，不重建菜单（避免关闭已打开的菜单）
    if let Ok(items_guard) = TRAY_MENU_ITEMS.lock() {
        if let Some(ref items) = *items_guard {
            let menu_text = TRAY_MENU_TEXT.lock().map_err(|e| e.to_string())?;

            let _ = items.show.set_text(&menu_text.show);
            let toggle_text = if is_running {
                &menu_text.stop
            } else {
                &menu_text.start
            };
            let _ = items.toggle.set_text(toggle_text);
            let _ = items.quit.set_text(&menu_text.quit);
            return Ok(());
        }
    }

    // 如果菜单项不存在（首次初始化），则完整创建菜单
    update_tray_menu(&app, is_running).map_err(|e| e.to_string())
}
