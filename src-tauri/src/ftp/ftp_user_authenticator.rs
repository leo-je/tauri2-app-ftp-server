//! FTP 用户认证模块
//!
//! 实现 FTP 服务器的用户认证逻辑，支持：
//! - 匿名访问
//! - 用户名/密码认证
//! - 基于权限的文件访问控制

use async_trait::async_trait;
use libunftp::auth::{AuthenticationError, Authenticator, Credentials};
use subtle::ConstantTimeEq;
use unftp_sbe_restrict::VfsOperations;

use crate::ftp::ftpuser::UserInfo;

/// FTP 用户认证器
///
/// 处理用户登录认证，验证用户名和密码
#[derive(Debug)]
pub struct FtpUserAuthenticator {
    /// 是否允许匿名访问
    pub is_anonymous: bool,
    /// 用户列表
    pub users: Vec<UserInfo>,
    /// 默认文件权限
    pub file_auth: String,
}

#[async_trait]
impl Authenticator<UserInfo> for FtpUserAuthenticator {
    /// 执行用户认证
    ///
    /// # 参数
    /// * `_username` - 用户名
    /// * `_creds` - 用户凭证（包含密码等）
    ///
    /// # 返回值
    /// * `Ok(UserInfo)` - 认证成功，返回用户信息
    /// * `Err(AuthenticationError)` - 认证失败
    ///
    /// # 认证流程
    /// 1. 如果允许匿名访问，直接返回成功
    /// 2. 在用户名列表中查找匹配的用户
    /// 3. 使用常量时间比较验证密码（防止时序攻击）
    /// 4. 根据配置设置用户权限
    async fn authenticate(
        &self,
        _username: &str,
        _creds: &Credentials,
    ) -> Result<UserInfo, AuthenticationError> {
        println!("Authenticating user {}\n", _username);
        // 匿名访问模式
        if self.is_anonymous {
            return Ok(UserInfo {
                username: _username.to_string(),
                password: "".to_string(),
                file_auth: "".to_string(),
                permissions: get_permissions(&self.file_auth),
            });
        }

        // 遍历用户列表进行认证
        for u in &self.users {
            if u.username == _username {
                println!("Authenticating user: {}-[REDACTED]", u.username);
                if let Some(password) = &_creds.password {
                    // 使用常量时间比较防止时序攻击
                    if password.as_bytes().ct_eq(u.password.as_bytes()).unwrap_u8() == 1 {
                        // 根据 file_auth 设置权限
                        let permissions = get_permissions(&u.file_auth);
                        return Ok(UserInfo {
                            username: _username.to_string(),
                            password: "".to_string(),
                            file_auth: "".to_string(),
                            permissions,
                        });
                    } else {
                        self.log_auth_failure(_username);
                        return Err(AuthenticationError::BadPassword);
                    }
                } else {
                    self.log_auth_failure(_username);
                    return Err(AuthenticationError::BadPassword);
                }
            }
        }
        self.log_auth_failure(_username);
        Err(AuthenticationError::BadPassword)
    }
}

impl FtpUserAuthenticator {
    /// 记录认证失败日志
    ///
    /// # 参数
    /// * `username` - 尝试登录的用户名
    fn log_auth_failure(&self, username: &str) {
        println!("username:{}用户不存在或者密码错误", username);
    }
}

/// 根据权限标识获取文件操作权限
///
/// # 参数
/// * `file_auth` - 权限标识字符串
///   - "W" - 读写权限
///   - 其他 - 只读权限（去除写操作）
///
/// # 返回值
/// 返回对应的 VfsOperations 权限集合
fn get_permissions(file_auth: &str) -> VfsOperations {
    println!("file_auth:{}", file_auth);
    let all_permissions = VfsOperations::all();
    match file_auth {
        "W" => all_permissions,
        _ => all_permissions - VfsOperations::WRITE_OPS,
    }
}
