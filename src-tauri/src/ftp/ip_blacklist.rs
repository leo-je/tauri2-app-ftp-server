//! IP黑名单管理模块
//!
//! 提供IP地址的黑名单功能，阻止指定IP访问FTP服务器

use std::collections::HashSet;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};

/// IP黑名单管理器
#[derive(Debug, Clone)]
pub struct IpBlacklist {
    /// 黑名单IP集合
    blocked_ips: Arc<Mutex<HashSet<String>>>,
}

impl Default for IpBlacklist {
    fn default() -> Self {
        Self::new()
    }
}

impl IpBlacklist {
    /// 创建新的IP黑名单管理器
    pub fn new() -> Self {
        Self {
            blocked_ips: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// 从字符串列表加载黑名单
    pub fn load_from_list(&self, ips: Vec<String>) {
        if let Ok(mut blocked) = self.blocked_ips.lock() {
            blocked.clear();
            for ip in ips {
                blocked.insert(ip);
            }
        }
    }

    /// 添加IP到黑名单
    pub fn add_ip(&self, ip: &str) -> Result<(), String> {
        // 验证IP地址格式
        match ip.parse::<IpAddr>() {
            Ok(_) => {
                if let Ok(mut blocked) = self.blocked_ips.lock() {
                    blocked.insert(ip.to_string());
                    Ok(())
                } else {
                    Err("无法获取黑名单锁".to_string())
                }
            }
            Err(_) => Err(format!("无效的IP地址: {}", ip)),
        }
    }

    /// 从黑名单移除IP
    pub fn remove_ip(&self, ip: &str) -> Result<(), String> {
        if let Ok(mut blocked) = self.blocked_ips.lock() {
            blocked.remove(ip);
            Ok(())
        } else {
            Err("无法获取黑名单锁".to_string())
        }
    }

    /// 检查IP是否在黑名单中
    pub fn is_blocked(&self, ip: &str) -> bool {
        if let Ok(blocked) = self.blocked_ips.lock() {
            // 检查精确匹配
            if blocked.contains(ip) {
                return true;
            }

            // 检查CIDR范围匹配
            for blocked_ip in blocked.iter() {
                if blocked_ip.contains('/') {
                    // CIDR格式
                    if Self::ip_in_cidr(ip, blocked_ip) {
                        return true;
                    }
                }
            }

            false
        } else {
            false
        }
    }

    /// 获取所有黑名单IP
    pub fn get_blocked_ips(&self) -> Vec<String> {
        if let Ok(blocked) = self.blocked_ips.lock() {
            blocked.iter().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// 清空黑名单
    pub fn clear(&self) {
        if let Ok(mut blocked) = self.blocked_ips.lock() {
            blocked.clear();
        }
    }

    /// 检查IP是否在CIDR范围内
    fn ip_in_cidr(ip: &str, cidr: &str) -> bool {
        let parts: Vec<&str> = cidr.split('/').collect();
        if parts.len() != 2 {
            return false;
        }

        let network_ip = match parts[0].parse::<IpAddr>() {
            Ok(ip) => ip,
            Err(_) => return false,
        };

        let prefix_len: u8 = match parts[1].parse() {
            Ok(n) if n <= 128 => n,
            _ => return false,
        };

        let check_ip = match ip.parse::<IpAddr>() {
            Ok(ip) => ip,
            Err(_) => return false,
        };

        match (network_ip, check_ip) {
            (IpAddr::V4(net), IpAddr::V4(check)) => {
                let net_bits = u32::from(net);
                let check_bits = u32::from(check);
                let mask = !((1u32 << (32 - prefix_len)) - 1);
                (net_bits & mask) == (check_bits & mask)
            }
            (IpAddr::V6(net), IpAddr::V6(check)) => {
                let net_bits = u128::from(net);
                let check_bits = u128::from(check);
                let mask = !((1u128 << (128 - prefix_len)) - 1);
                (net_bits & mask) == (check_bits & mask)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_check_ip() {
        let blacklist = IpBlacklist::new();
        assert!(!blacklist.is_blocked("192.168.1.1"));

        blacklist.add_ip("192.168.1.1").unwrap();
        assert!(blacklist.is_blocked("192.168.1.1"));
    }

    #[test]
    fn test_cidr_blocking() {
        let blacklist = IpBlacklist::new();
        blacklist.add_ip("192.168.0.0/24").unwrap();

        assert!(blacklist.is_blocked("192.168.0.1"));
        assert!(blacklist.is_blocked("192.168.0.100"));
        assert!(!blacklist.is_blocked("192.168.1.1"));
    }
}
