use libunftp::auth::UserDetail;
use std::fmt::Formatter;
use unftp_sbe_restrict::{UserWithPermissions, VfsOperations};



#[derive(Debug, PartialEq, Eq, serde::Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
    pub fileauth: String,
    #[serde(skip,default = "VfsOperations::all")]
    pub permissions: VfsOperations,
}

impl UserDetail for UserInfo {
    fn account_enabled(&self) -> bool {
        true
    }
}

impl std::fmt::Display for UserInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User(username: {:?}", self.username,)
    }
}

impl UserWithPermissions for UserInfo {
    fn permissions(&self) -> VfsOperations {
        self.permissions
    }
}

// Return type omited for brevity.
pub fn create_restricted_storage_backend(
    ftp_home: String,
) -> Box<
    dyn Fn() -> unftp_sbe_restrict::RestrictingVfs<
        unftp_sbe_fs::Filesystem,
        UserInfo,
        unftp_sbe_fs::Meta,
    >,
> {
    use unftp_sbe_fs::{Filesystem, Meta};
    let backend = Box::new(move || {
        unftp_sbe_restrict::RestrictingVfs::<Filesystem, UserInfo, Meta>::new(Filesystem::new(
            ftp_home.clone(),
        ))
    });
    backend
}
