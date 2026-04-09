//! IP黑名单管理命令
//!
//! 提供前端可调用的命令，用于管理IP黑名单

use std::sync::{Arc, Mutex};
use tauri::State;

use crate::ftp::ip_blacklist::IpBlacklist;

/// IP黑名单状态
pub struct IpBlacklistState {
    pub blacklist: Arc<Mutex<IpBlacklist>>,
}

impl Default for IpBlacklistState {
    fn default() -> Self {
        Self {
            blacklist: Arc::new(Mutex::new(IpBlacklist::new())),
        }
    }
}

/// 添加IP到黑名单
#[tauri::command]
pub fn add_ip_to_blacklist(
    state: State<'_, Arc<Mutex<IpBlacklist>>>,
    ip: String,
) -> Result<(), String> {
    if let Ok(blacklist) = state.lock() {
        blacklist.add_ip(&ip)
    } else {
        Err("无法获取黑名单锁".to_string())
    }
}

/// 从黑名单移除IP
#[tauri::command]
pub fn remove_ip_from_blacklist(
    state: State<'_, Arc<Mutex<IpBlacklist>>>,
    ip: String,
) -> Result<(), String> {
    if let Ok(blacklist) = state.lock() {
        blacklist.remove_ip(&ip)
    } else {
        Err("无法获取黑名单锁".to_string())
    }
}

/// 获取所有黑名单IP
#[tauri::command]
pub fn get_blacklist_ips(state: State<'_, Arc<Mutex<IpBlacklist>>>) -> Vec<String> {
    if let Ok(blacklist) = state.lock() {
        blacklist.get_blocked_ips()
    } else {
        Vec::new()
    }
}

/// 检查IP是否在黑名单中
#[tauri::command]
pub fn is_ip_blacklisted(state: State<'_, Arc<Mutex<IpBlacklist>>>, ip: String) -> bool {
    if let Ok(blacklist) = state.lock() {
        blacklist.is_blocked(&ip)
    } else {
        false
    }
}

/// 清空黑名单
#[tauri::command]
pub fn clear_blacklist(state: State<'_, Arc<Mutex<IpBlacklist>>>) -> Result<(), String> {
    if let Ok(blacklist) = state.lock() {
        blacklist.clear();
        Ok(())
    } else {
        Err("无法获取黑名单锁".to_string())
    }
}

/// 加载黑名单列表
#[tauri::command]
pub fn load_blacklist(
    state: State<'_, Arc<Mutex<IpBlacklist>>>,
    ips: Vec<String>,
) -> Result<(), String> {
    if let Ok(blacklist) = state.lock() {
        blacklist.load_from_list(ips);
        Ok(())
    } else {
        Err("无法获取黑名单锁".to_string())
    }
}
