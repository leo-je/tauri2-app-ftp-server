use libunftp::auth::UserDetail;
use std::fmt::Formatter;
use unftp_sbe_restrict::{UserWithPermissions, VfsOperations};



#[derive(Debug, PartialEq, Eq, serde::Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
    pub fileauth: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub username: String,
    // e.g. this can be something like
    // `VfsOperations::all() - VfsOperations::PUT - VfsOperations::DEL`
    pub permissions: VfsOperations,
}

impl UserDetail for User {
    fn account_enabled(&self) -> bool {
        true
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User(username: {:?}", self.username,)
    }
}

impl UserWithPermissions for User {
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
        User,
        unftp_sbe_fs::Meta,
    >,
> {
    use unftp_sbe_fs::{Filesystem, Meta};
    let backend = Box::new(move || {
        unftp_sbe_restrict::RestrictingVfs::<Filesystem, User, Meta>::new(Filesystem::new(
            ftp_home.clone(),
        ))
    });
    backend
}
