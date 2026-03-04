//! FTP 用户信息模块
//!
//! 定义 FTP 用户的数据结构和相关 trait 实现，
//! 包括用户认证信息、权限控制等功能。

use libunftp::auth::UserDetail;
use std::fmt::Formatter;
use unftp_sbe_restrict::{UserWithPermissions, VfsOperations};

/// FTP 用户信息结构体
///
/// 存储用户的认证信息和文件操作权限
#[derive(Debug, PartialEq, Eq, serde::Deserialize)]
pub struct UserInfo {
    /// 用户名
    pub username: String,
    /// 密码（明文存储，生产环境建议使用哈希）
    pub password: String,
    /// 文件权限标识（"W"=读写，其他=只读）
    #[serde(default = "String::new",alias = "fileAuth")]
    pub file_auth: String,
    /// 虚拟文件系统操作权限
    #[serde(skip, default = "VfsOperations::all")]
    pub permissions: VfsOperations,
}

impl UserDetail for UserInfo {
    /// 检查用户账户是否启用
    ///
    /// # 返回值
    /// 总是返回 `true`，表示所有账户都可用
    fn account_enabled(&self) -> bool {
        true
    }
}

impl std::fmt::Display for UserInfo {
    /// 格式化输出用户信息（隐藏密码）
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User(username: {:?}", self.username,)
    }
}

impl UserWithPermissions for UserInfo {
    /// 获取用户的文件操作权限
    fn permissions(&self) -> VfsOperations {
        self.permissions
    }
}
