//! 存储层操作日志
//!
//! 这里只记录 **CWD**（切换目录）——libunftp 的 `DataEvent` 没有对应的事件，
//! 只有包在存储层上才能看到。
//!
//! 其余六种操作（download / upload / delete / mkdir / rmdir / rename）
//! 统一交给 `FtpDataListener` 记录：那边带得到字节数和真实用户名，
//! 而本层拿不到字节数。两边都记的话，每一次传输都会在日志里出现两行
//! （一行没有大小、一行有），计数器也跟着翻倍。

use async_trait::async_trait;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use unftp_core::storage::{Fileinfo, StorageBackend, Result as StorageResult};
use tokio::io::AsyncRead;
use crate::ftp::ftpevent::FtpEventLogger;
use std::sync::Mutex;
use crate::ftp::ftpuser::UserInfo;

pub struct LoggingVfs<Inner> {
    inner: Inner,
    logger: Option<Arc<Mutex<FtpEventLogger>>>,
    current_user: std::sync::Mutex<Option<String>>,
}

impl<Inner> LoggingVfs<Inner> {
    pub fn new(inner: Inner, logger: Option<Arc<Mutex<FtpEventLogger>>>) -> Self {
        Self {
            inner,
            logger,
            current_user: std::sync::Mutex::new(None),
        }
    }

    pub fn set_current_user(&self, username: Option<String>) {
        if let Ok(mut guard) = self.current_user.lock() {
            *guard = username;
        }
    }

    fn log_operation(&self, operation: &str, path: &Path, extra: Option<String>) {
        let Some(ref logger) = self.logger else {
            return;
        };

        let username = self.current_user
            .lock()
            .ok()
            .and_then(|g| g.clone())
            .unwrap_or_else(|| "anonymous".to_string());

        let log = crate::ftp::ftpevent::FtpOperationLog::new(
            operation,
            path.to_string_lossy().as_ref(),
            None,
            Some(username),
            extra,
        );

        if let Ok(logger_guard) = logger.lock() {
            logger_guard.add_log(log);
        }
    }
}

impl<Inner> Debug for LoggingVfs<Inner>
where
    Inner: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoggingVfs")
            .field("inner", &self.inner)
            .finish()
    }
}

#[async_trait]
impl<Inner> StorageBackend<UserInfo> for LoggingVfs<Inner>
where
    Inner: StorageBackend<UserInfo> + Send + Sync,
{
    type Metadata = Inner::Metadata;

    async fn metadata<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &UserInfo,
        path: P,
    ) -> StorageResult<Self::Metadata> {
        self.inner.metadata(user, path).await
    }

    async fn list<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &UserInfo,
        path: P,
    ) -> StorageResult<Vec<Fileinfo<PathBuf, Self::Metadata>>> {
        self.inner.list(user, path).await
    }

    async fn get<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &UserInfo,
        path: P,
        start_pos: u64,
    ) -> StorageResult<Box<dyn AsyncRead + Send + Sync + Unpin>> {
        self.inner.get(user, path, start_pos).await
    }

    async fn put<P: AsRef<Path> + Send + Debug, R: AsyncRead + Send + Sync + Unpin + 'static>(
        &self,
        user: &UserInfo,
        input: R,
        path: P,
        start_pos: u64,
    ) -> StorageResult<u64> {
        self.inner.put(user, input, path, start_pos).await
    }

    async fn del<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &UserInfo,
        path: P,
    ) -> StorageResult<()> {
        self.inner.del(user, path).await
    }

    async fn mkd<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &UserInfo,
        path: P,
    ) -> StorageResult<()> {
        self.inner.mkd(user, path).await
    }

    async fn rmd<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &UserInfo,
        path: P,
    ) -> StorageResult<()> {
        self.inner.rmd(user, path).await
    }

    async fn cwd<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &UserInfo,
        path: P,
    ) -> StorageResult<()> {
        // 只有这一处需要自己记：DataEvent 里没有「切换目录」事件
        self.set_current_user(Some(user.username.clone()));

        let path_for_log = path.as_ref().to_path_buf();
        self.log_operation("cwd", &path_for_log, None);

        self.inner.cwd(user, path).await
    }

    async fn rename<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &UserInfo,
        from: P,
        to: P,
    ) -> StorageResult<()> {
        self.inner.rename(user, from, to).await
    }
}

pub fn create_logging_vfs<Inner>(
    inner: Inner,
    logger: Option<Arc<Mutex<FtpEventLogger>>>,
) -> LoggingVfs<Inner> {
    LoggingVfs::new(inner, logger)
}
