//! FTP 文件操作事件模块
//!
//! 该模块负责处理 FTP 服务器的文件操作事件，包括：
//! - 文件下载 (RETR)
//! - 文件上传 (STOR)
//! - 文件删除 (DELE)
//! - 目录创建 (MKD)
//! - 目录删除 (RMD)
//! - 文件/目录重命名 (RNFR/RNTO)

use async_trait::async_trait;
use chrono::Local;
use libunftp::notification::{DataEvent, DataListener, EventMeta};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

/// FTP 操作日志条目
///
/// 记录单个 FTP 文件操作的所有相关信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FtpOperationLog {
    /// 操作时间（格式：YYYY-MM-DD HH:mm:ss）
    pub time: String,
    /// 操作类型（download/upload/delete/mkdir/rmdir/rename）
    pub operation: String,
    /// 涉及的文件路径
    pub path: String,
    /// 传输字节数（下载/上传时有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
    /// 用户名（如果已知）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// 附加信息（如重命名时的原路径）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

impl FtpOperationLog {
    /// 创建新的日志条目
    pub fn new(
        operation: &str,
        path: &str,
        bytes: Option<u64>,
        username: Option<String>,
        extra: Option<String>,
    ) -> Self {
        Self {
            time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            operation: operation.to_string(),
            path: path.to_string(),
            bytes,
            username,
            extra,
        }
    }
}

/// FTP 操作日志管理器
///
/// 负责存储和管理 FTP 操作日志，并通过 Tauri 事件将日志推送到前端
pub struct FtpEventLogger {
    /// 日志存储（最多保留 1000 条）
    logs: Arc<Mutex<Vec<FtpOperationLog>>>,
    /// 最大日志条数
    max_logs: usize,
    /// Tauri 应用句柄
    app_handle: Option<AppHandle>,
}

impl Default for FtpEventLogger {
    fn default() -> Self {
        Self::new(1000)
    }
}

impl FtpEventLogger {
    /// 创建新的日志管理器
    ///
    /// # 参数
    /// * `max_logs` - 最大保留日志条数
    pub fn new(max_logs: usize) -> Self {
        Self {
            logs: Arc::new(Mutex::new(Vec::new())),
            max_logs,
            app_handle: None,
        }
    }

    /// 设置 Tauri 应用句柄
    pub fn set_app_handle(&mut self, handle: AppHandle) {
        self.app_handle = Some(handle);
    }

    /// 添加日志条目
    pub fn add_log(&self, log: FtpOperationLog) {
        let mut logs = match self.logs.lock() {
            Ok(guard) => guard,
            Err(_) => return,
        };

        while logs.len() >= self.max_logs {
            logs.remove(0);
        }

        logs.push(log.clone());
        drop(logs);

        if let Some(ref handle) = self.app_handle {
            let _ = handle.emit("ftp-operation-log", log);
        }
    }

    pub fn get_logs(&self) -> Vec<FtpOperationLog> {
        self.logs.lock().map(|g| g.clone()).unwrap_or_default()
    }

    pub fn clear_logs(&self) {
        if let Ok(mut logs) = self.logs.lock() {
            logs.clear();
        }
    }

    pub fn log_count(&self) -> usize {
        self.logs.lock().map(|g| g.len()).unwrap_or(0)
    }
}

impl Debug for FtpEventLogger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FtpEventLogger")
            .field("log_count", &self.log_count())
            .field("max_logs", &self.max_logs)
            .finish()
    }
}

/// DataListener 实现，用于接收 FTP 文件操作事件
pub struct FtpDataListener {
    /// 日志管理器
    logger: Arc<Mutex<FtpEventLogger>>,
}

impl FtpDataListener {
    /// 创建新的 FTP 数据监听器
    pub fn new(logger: Arc<Mutex<FtpEventLogger>>) -> Self {
        Self { logger }
    }
}

impl Debug for FtpDataListener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FtpDataListener").finish()
    }
}

#[async_trait]
impl DataListener for FtpDataListener {
    /// 接收 FTP 数据事件
    async fn receive_data_event(&self, event: DataEvent, meta: EventMeta) {
        let log = match event {
            DataEvent::Got { path, bytes } => FtpOperationLog::new(
                "download",
                &path,
                Some(bytes),
                Some(meta.username),
                None,
            ),
            DataEvent::Put { path, bytes } => FtpOperationLog::new(
                "upload",
                &path,
                Some(bytes),
                Some(meta.username),
                None,
            ),
            DataEvent::Deleted { path } => FtpOperationLog::new(
                "delete",
                &path,
                None,
                Some(meta.username),
                None,
            ),
            DataEvent::MadeDir { path } => FtpOperationLog::new(
                "mkdir",
                &path,
                None,
                Some(meta.username),
                None,
            ),
            DataEvent::RemovedDir { path } => FtpOperationLog::new(
                "rmdir",
                &path,
                None,
                Some(meta.username),
                None,
            ),
            DataEvent::Renamed { from, to } => FtpOperationLog::new(
                "rename",
                &to,
                None,
                Some(meta.username),
                Some(from),
            ),
        };

        if let Some(logger) = self.logger.lock().ok() {
            logger.add_log(log);
        }
    }
}
