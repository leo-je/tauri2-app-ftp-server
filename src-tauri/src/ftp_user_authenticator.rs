use async_trait::async_trait;
use libunftp::auth::{AuthenticationError, Authenticator, Credentials};
use subtle::ConstantTimeEq;
use unftp_sbe_restrict::VfsOperations;

use crate::ftpuser::{AuthUser, UserInfo};

#[derive(Debug)]
pub struct FtpUserAuthenticator {
    pub is_anonymous: bool,
    pub users: Vec<UserInfo>,
    pub fileauth:String,
}

#[async_trait]
impl Authenticator<AuthUser> for FtpUserAuthenticator {
    async fn authenticate(
        &self,
        _username: &str,
        _creds: &Credentials,
    ) -> Result<AuthUser, AuthenticationError> {
        println!("Authenticating user {}\n", _username);
        if self.is_anonymous {
            return Ok(AuthUser {
                username: _username.to_string(),
                permissions: get_permissions(&self.fileauth),
            });
        }

        for u in &self.users {
            if u.username == _username {
                println!("Authenticating user: {}-{}", u.username, "[REDACTED]");
                if let Some(password) = &_creds.password {
                    if password.as_bytes().ct_eq(u.password.as_bytes()).unwrap_u8() == 1 {
                        // 根据u.file_auth设置权限
                        let permissions= get_permissions(&u.fileauth);
                        return Ok(AuthUser {
                            username: _username.to_string(),
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
    fn log_auth_failure(&self, username: &str) {
        println!("username:{}用户不存在或者密码错误", username);
    }
}

fn get_permissions(file_auth: &str) -> VfsOperations {
    println!("file_auth:{}", file_auth);
    let all_permissions = VfsOperations::all();
    match file_auth {
        "W" => all_permissions,
        _ => all_permissions - VfsOperations::WRITE_OPS,
    }
}