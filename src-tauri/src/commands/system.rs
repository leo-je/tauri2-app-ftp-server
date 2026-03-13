//! 系统信息和权限检查命令
//!
//! 包含系统信息获取、应用配置检查、权限检查和服务器运行状态管理的 Tauri 命令。

use std::sync::{Arc, Mutex};

use crate::AppState;
use crate::validators::{check_write_permission, check_privileged_port_permission};
use serde::{Deserialize, Serialize};
use tauri_plugin_os::{arch, platform, version};

/// 系统信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub platform: String,
    pub arch: String,
    pub version: String,
    pub hostname: String,
}

/// 配置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub config_exists: bool,
    pub config_valid: bool,
    pub default_path: Option<String>,
    pub default_port: u16,
}

/// 权限检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionCheck {
    pub has_write_permission: bool,
    pub has_network_permission: bool,
    pub can_bind_privileged_port: bool,
}

/// 获取系统信息
///
/// 检测当前操作系统、架构和版本信息
#[tauri::command]
pub fn get_system_info() -> Result<SystemInfo, String> {
    let platform_str = platform().to_string();
    let arch_str = arch().to_string();
    let version_str = version().to_string();

    // 获取主机名 - 使用环境变量
    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .or_else(|_| std::env::var("NAME"))
        .unwrap_or_else(|_| "unknown".to_string());

    Ok(SystemInfo {
        platform: platform_str,
        arch: arch_str,
        version: version_str,
        hostname,
    })
}

/// 检查应用配置
///
/// 检查配置文件是否存在且有效
#[tauri::command]
pub fn check_app_config(app_handle: tauri::AppHandle) -> Result<AppConfig, String> {
    use tauri_plugin_store::StoreExt;

    // 尝试获取存储
    let store = app_handle.store("app-config.json")
        .map_err(|e| format!("无法访问配置存储: {}", e))?;

    // 检查是否有默认配置
    let default_path = store.get("defaultPath")
        .and_then(|v| v.as_str().map(|s| s.to_string()));

    let default_port = store.get("defaultPort")
        .and_then(|v| v.as_u64())
        .map(|p| p as u16)
        .unwrap_or(2121);

    let config_exists = default_path.is_some();

    Ok(AppConfig {
        config_exists,
        config_valid: config_exists,
        default_path,
        default_port,
    })
}

/// 检查权限状态
///
/// 检查应用运行所需的各项权限
#[tauri::command]
pub fn check_permissions() -> Result<PermissionCheck, String> {
    let has_write_permission = check_write_permission();
    let has_network_permission = true; // Tauri 应用中网络权限通常是默认的
    let can_bind_privileged_port = check_privileged_port_permission();

    Ok(PermissionCheck {
        has_write_permission,
        has_network_permission,
        can_bind_privileged_port,
    })
}

/// 设置服务运行状态
#[tauri::command]
pub fn set_server_running(
    app: tauri::AppHandle,
    app_state: tauri::State<'_, Arc<Mutex<AppState>>>,
    running: bool,
) -> Result<(), String> {
    // 更新状态
    if let Ok(state) = app_state.lock() {
        if let Ok(mut is_running) = state.is_server_running.lock() {
            *is_running = running;
        }
    }

    // 更新托盘菜单
    crate::update_tray_menu(&app, running)
        .map_err(|e| format!("更新托盘菜单失败: {}", e))?;

    Ok(())
}

/// 获取服务运行状态
#[tauri::command]
pub fn get_server_running(
    app_state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<bool, String> {
    if let Ok(state) = app_state.lock() {
        if let Ok(is_running) = state.is_server_running.lock() {
            return Ok(*is_running);
        }
    }
    Ok(false)
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
            window.set_skip_taskbar(true)
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
            window.set_skip_taskbar(false)
                .map_err(|e| format!("显示任务栏图标失败: {}", e))?;
        }
    }
    Ok(())
}
