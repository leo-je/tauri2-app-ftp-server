use libunftp::auth::{AuthenticationError, Authenticator, Credentials};
use unftp_sbe_restrict::VfsOperations;
use async_trait::async_trait;
use subtle::ConstantTimeEq;

use crate::ftpuser::{User, UserInfo};

#[derive(Debug)]
pub struct FtpUserAuthenticator {
    pub is_anonymous: bool,
    pub users: Vec<UserInfo>,
}

#[async_trait]
impl Authenticator<User> for FtpUserAuthenticator {
    async fn authenticate(
        &self,
        _username: &str,
        _creds: &Credentials,
    ) -> Result<User, AuthenticationError> {
        println!("Authenticating user {}\n", _username);
        if self.is_anonymous {
            return Ok(User {
                username: _username.to_string(),
                permissions: VfsOperations::all(),
            });
        }

        for u in &self.users {
            if u.username == _username {
                println!("Authenticating user: {}-{}", u.username, "[REDACTED]");
                if let Some(password) = &_creds.password {
                    if password.as_bytes().ct_eq(u.password.as_bytes()).unwrap_u8() == 1 {
                        return Ok(User {
                            username: _username.to_string(),
                            permissions: VfsOperations::all(),
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