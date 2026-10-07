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
    ///
    /// 内容来自前端持久化的 store.json，属于外部输入，所以逐条校验：
    /// 非法条目跳过而不是入库——否则后续匹配时要额外防越界前缀。
    pub fn load_from_list(&self, ips: Vec<String>) {
        if let Ok(mut blocked) = self.blocked_ips.lock() {
            blocked.clear();
            for ip in ips {
                match Self::normalize_entry(&ip) {
                    Ok(normalized) => {
                        blocked.insert(normalized);
                    }
                    Err(e) => eprintln!("跳过无效的黑名单条目 {ip:?}: {e}"),
                }
            }
        }
    }

    /// 校验并规范化一条黑名单条目
    ///
    /// 同时接受单个 IP 和 CIDR 网段（`192.168.0.0/24`），这是界面上写明的两种格式。
    /// 地址部分统一走 `IpAddr` 的 Display 输出，避免同一个地址因写法不同存成两条。
    pub fn normalize_entry(entry: &str) -> Result<String, String> {
        let entry = entry.trim();
        if entry.is_empty() {
            return Err("IP地址不能为空".to_string());
        }

        // 单个 IP
        if let Ok(ip) = entry.parse::<IpAddr>() {
            return Ok(ip.to_string());
        }

        // CIDR 网段
        let (addr, prefix) = entry
            .split_once('/')
            .ok_or_else(|| format!("无效的IP地址: {entry}"))?;
        let ip: IpAddr = addr
            .trim()
            .parse()
            .map_err(|_| format!("无效的IP地址: {entry}"))?;
        let prefix_text = prefix.trim();
        let prefix: u8 = prefix_text
            .parse()
            .map_err(|_| format!("无效的前缀长度: {prefix_text}"))?;

        let max_prefix = match ip {
            IpAddr::V4(_) => 32,
            IpAddr::V6(_) => 128,
        };
        if prefix > max_prefix {
            return Err(format!("前缀长度必须在 0-{max_prefix} 之间: {prefix}"));
        }

        Ok(format!("{ip}/{prefix}"))
    }

    /// 添加IP或CIDR网段到黑名单
    pub fn add_ip(&self, ip: &str) -> Result<(), String> {
        let normalized = Self::normalize_entry(ip)?;
        if let Ok(mut blocked) = self.blocked_ips.lock() {
            blocked.insert(normalized);
            Ok(())
        } else {
            Err("无法获取黑名单锁".to_string())
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
        let Some((addr, prefix)) = cidr.split_once('/') else {
            return false;
        };
        let (Ok(network), Ok(prefix)) = (addr.trim().parse::<IpAddr>(), prefix.trim().parse::<u8>())
        else {
            return false;
        };
        let Ok(check) = ip.parse::<IpAddr>() else {
            return false;
        };

        match (network, check) {
            (IpAddr::V4(net), IpAddr::V4(check)) => {
                if prefix > 32 {
                    return false;
                }
                // 前缀 0 的掩码是全 0。不能写成 `!0 << 32`——移位量等于位宽会 panic
                let mask = if prefix == 0 { 0 } else { u32::MAX << (32 - prefix) };
                (u32::from(net) & mask) == (u32::from(check) & mask)
            }
            (IpAddr::V6(net), IpAddr::V6(check)) => {
                if prefix > 128 {
                    return false;
                }
                let mask = if prefix == 0 {
                    0
                } else {
                    u128::MAX << (128 - prefix)
                };
                (u128::from(net) & mask) == (u128::from(check) & mask)
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

    #[test]
    fn test_normalize_entry() {
        // 单个 IP 与 CIDR 都接受
        assert_eq!(IpBlacklist::normalize_entry("10.0.0.1").unwrap(), "10.0.0.1");
        assert_eq!(
            IpBlacklist::normalize_entry(" 10.0.0.0/8 ").unwrap(),
            "10.0.0.0/8"
        );
        assert_eq!(IpBlacklist::normalize_entry("::1").unwrap(), "::1");

        // 垃圾输入与越界前缀必须被拒
        assert!(IpBlacklist::normalize_entry("").is_err());
        assert!(IpBlacklist::normalize_entry("not-an-ip").is_err());
        assert!(IpBlacklist::normalize_entry("10.0.0.0/33").is_err());
        assert!(IpBlacklist::normalize_entry("10.0.0.0/").is_err());
        assert!(IpBlacklist::normalize_entry("::1/129").is_err());
    }

    #[test]
    fn test_zero_prefix_does_not_panic() {
        // /0 的掩码是全 0：旧实现写的是 `!((1 << 32) - 1)`，移位量等于位宽会 panic
        let blacklist = IpBlacklist::new();
        blacklist.add_ip("0.0.0.0/0").unwrap();
        assert!(blacklist.is_blocked("8.8.8.8"));
        assert!(blacklist.is_blocked("192.168.1.1"));

        let v6 = IpBlacklist::new();
        v6.add_ip("::/0").unwrap();
        assert!(v6.is_blocked("2001:db8::1"));
    }

    #[test]
    fn test_load_from_list_skips_invalid() {
        let blacklist = IpBlacklist::new();
        blacklist.load_from_list(vec![
            "192.168.1.1".to_string(),
            "garbage".to_string(),
            "10.0.0.0/8".to_string(),
            "10.0.0.0/99".to_string(),
        ]);

        assert_eq!(blacklist.get_blocked_ips().len(), 2);
        assert!(blacklist.is_blocked("192.168.1.1"));
        assert!(blacklist.is_blocked("10.1.2.3"));
    }
}
