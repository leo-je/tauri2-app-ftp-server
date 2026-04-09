//! FTP 用户认证模块
//!
//! 实现 FTP 服务器的用户认证逻辑，支持：
//! - 匿名访问
//! - 用户名/密码认证
//! - 基于权限的文件访问控制

use async_trait::async_trait;
use unftp_core::auth::{AuthenticationError, Authenticator, Credentials, Principal, UserDetailProvider, UserDetailError};
use subtle::ConstantTimeEq;
use unftp_sbe_restrict::VfsOperations;
use std::sync::{Arc, Mutex};

use crate::ftp::ftpuser::UserInfo;
use crate::ftp::ftpevent::FtpEventLogger;

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
    /// 日志管理器
    pub logger: Option<Arc<Mutex<FtpEventLogger>>>,
}

#[async_trait]
impl Authenticator for FtpUserAuthenticator {
    async fn authenticate(
        &self,
        _username: &str,
        _creds: &Credentials,
    ) -> Result<Principal, AuthenticationError> {
        // 匿名访问模式
        if self.is_anonymous {
            self.log_login(_username, true);
            return Ok(Principal {
                username: _username.to_string(),
            });
        }

        // 遍历用户列表进行认证
        for u in &self.users {
            if u.username == _username {
                if let Some(password) = &_creds.password {
                    if password.as_bytes().ct_eq(u.password.as_bytes()).unwrap_u8() == 1 {
                        self.log_login(_username, true);
                        return Ok(Principal {
                            username: _username.to_string(),
                        });
                    } else {
                        self.log_auth_failure(_username);
                        self.log_login(_username, false);
                        return Err(AuthenticationError::BadPassword);
                    }
                } else {
                    self.log_auth_failure(_username);
                    self.log_login(_username, false);
                    return Err(AuthenticationError::BadPassword);
                }
            }
        }
        self.log_auth_failure(_username);
        self.log_login(_username, false);
        Err(AuthenticationError::BadPassword)
    }
}

impl FtpUserAuthenticator {
    fn log_auth_failure(&self, username: &str) {
        // 仅记录认证失败事件，不泄露密码等敏感信息
        tracing::debug!("FTP auth failed for user: {}", username);
    }

    fn log_login(&self, username: &str, success: bool) {
        if let Some(ref logger) = self.logger {
            if let Ok(logger_guard) = logger.lock() {
                let status = if success { "success" } else { "failed" };
                let log = crate::ftp::ftpevent::FtpOperationLog::new(
                    "login",
                    &format!("User: {}", username),
                    None,
                    Some(username.to_string()),
                    Some(format!("Status: {}", status)),
                );
                logger_guard.add_log(log);
            }
        }
    }
}

/// FTP 用户详情提供者
///
/// 将认证后的 Principal 转换为完整的 UserInfo
#[derive(Debug)]
pub struct FtpUserDetailProvider {
    pub users: Vec<UserInfo>,
    pub file_auth: String,
}

#[async_trait]
impl UserDetailProvider for FtpUserDetailProvider {
    type User = UserInfo;

    async fn provide_user_detail(&self, principal: &Principal) -> Result<UserInfo, UserDetailError> {
        // 查找用户
        for u in &self.users {
            if u.username == principal.username {
                let permissions = get_permissions(&u.file_auth);
                return Ok(UserInfo {
                    username: principal.username.clone(),
                    password: "".to_string(),
                    file_auth: "".to_string(),
                    permissions,
                });
            }
        }
        
        // 匿名用户或默认用户
        if principal.username.is_empty() || self.users.is_empty() {
            let permissions = get_permissions(&self.file_auth);
            return Ok(UserInfo {
                username: principal.username.clone(),
                password: "".to_string(),
                file_auth: "".to_string(),
                permissions,
            });
        }
        
        Err(UserDetailError::UserNotFound { username: principal.username.clone() })
    }
}

fn get_permissions(file_auth: &str) -> VfsOperations {
    let all_permissions = VfsOperations::all();
    match file_auth {
        "W" => all_permissions,
        _ => all_permissions - VfsOperations::WRITE_OPS,
    }
}
