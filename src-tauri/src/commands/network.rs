//! 网络接口命令
//!
//! 包含获取网络接口和 IPv4 地址的 Tauri 命令。

use get_if_addrs::get_if_addrs;
use std::net::IpAddr;

/// 获取网络接口信息
#[tauri::command]
pub fn get_network_interfaces() -> Result<Vec<String>, String> {
    let mut interfaces = vec![];

    if let Ok(ifaces) = get_if_addrs() {
        for iface in ifaces {
            if let IpAddr::V4(ipv4) = iface.ip() {
                if !ipv4.is_loopback() {
                    interfaces.push(format!("{}: {}", iface.name, ipv4));
                }
            }
        }
    }

    Ok(interfaces)
}

/// 获取本机主 IPv4 地址列表（Tauri 命令）
///
/// 遍历所有网络接口，返回非回环的 IPv4 地址列表。
/// 用于前端显示可供连接的 FTP 服务器地址。
///
/// # 返回值
/// 返回 IPv4 地址字符串数组，例如：["192.168.1.100", "10.0.0.5"]
#[tauri::command]
pub fn get_primary_ipv4() -> Vec<String> {
    let mut result = vec![];

    if let Ok(ifaces) = get_if_addrs() {
        for iface in ifaces {
            if let IpAddr::V4(ipv4) = iface.ip() {
                // 过滤掉回环地址（127.0.0.1）
                if !ipv4.is_loopback() {
                    result.push(ipv4.to_string());
                }
            }
        }
    }

    result
}
