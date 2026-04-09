//! FTP 工作线程模块
//!
//! 该模块负责管理 FTP 服务器的生命周期，包括启动和停止 FTP 服务。
//! 使用独立的线程运行 FTP 服务器，避免阻塞主线程。

use crate::ftp::ftpevent::{FtpDataListener, FtpEventLogger};
use crate::ftp::{ftp_user_authenticator::{FtpUserAuthenticator, FtpUserDetailProvider}, ftpuser::UserInfo};
use std::{
    net::{SocketAddr, TcpListener},
    ops::RangeInclusive,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex,
    },
    thread,
    time::Duration,
};
use tokio::runtime::Runtime;
use unftp_sbe_fs::{Filesystem, Meta};

/// 默认被动模式端口范围起始值
const DEFAULT_PASSIVE_PORT_START: u16 = 50000;
/// 默认被动模式端口范围结束值
const DEFAULT_PASSIVE_PORT_END: u16 = 65535;

/// FTP 服务器配置结构体
///
/// 包含启动 FTP 服务器所需的所有配置参数
#[derive(Clone, Debug)]
pub struct FtpWorkerConfig {
    /// FTP 根目录路径
    pub path: String,
    /// FTP 服务监听端口
    pub port: String,
    /// 用户列表（JSON 格式字符串）
    pub users: String,
    /// 是否允许匿名访问
    pub is_anonymous: bool,
    /// 文件权限设置（"W" 表示读写，其他表示只读）
    pub file_auth: String,
    /// 被动模式端口范围
    pub passive_port_range: RangeInclusive<u16>,
}

impl Default for FtpWorkerConfig {
    fn default() -> Self {
        Self {
            path: "/default/path".to_string(),
            port: "2121".to_string(),
            users: "".to_string(),
            is_anonymous: true,
            file_auth: "R".to_string(),
            passive_port_range: DEFAULT_PASSIVE_PORT_START..=DEFAULT_PASSIVE_PORT_END,
        }
    }
}

/// FTP 工作线程结构体
///
/// 管理 FTP 服务器的运行状态，包括配置、线程句柄和运行标志
pub struct FtpWorker {
    /// FTP 服务器配置
    pub config: FtpWorkerConfig,
    /// FTP 服务线程句柄
    pub handle: Option<thread::JoinHandle<()>>,
    /// 运行状态标志（线程安全）
    running: Arc<AtomicBool>,
    /// FTP 事件日志管理器
    logger: Arc<Mutex<Option<std::sync::Arc<Mutex<FtpEventLogger>>>>>,
}

impl Default for FtpWorker {
    fn default() -> Self {
        Self::new()
    }
}

impl FtpWorker {
    /// 创建新的 FTP 工作线程实例
    pub fn new() -> Self {
        let running = Arc::new(AtomicBool::new(false));
        FtpWorker {
            handle: None,
            config: FtpWorkerConfig::default(),
            running,
            logger: Arc::new(Mutex::new(None)),
        }
    }

    /// 设置 FTP 服务器配置
    pub fn set(&mut self, config: FtpWorkerConfig) {
        self.config = config;
    }

    /// 设置 FTP 事件日志管理器
    pub fn set_logger(&self, logger: std::sync::Arc<Mutex<FtpEventLogger>>) {
        if let Ok(mut guard) = self.logger.lock() {
            *guard = Some(logger);
        }
    }
}

impl FtpWorker {
    /// 启动 FTP 服务器
    ///
    /// 在新线程中启动 FTP 服务器，使用 Tokio 运行时处理异步操作。
    /// 如果服务器已经在运行，则不会重复启动。
    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.reap_finished_handle()?;

        if self.is_running() || self.handle.is_some() {
            return Ok(());
        }

        let bind_address = bind_address(&self.config.port)?;
        let bind_address_str = bind_address.to_string();
        ensure_bind_available(bind_address)?;

        let (tx, rx) = mpsc::channel::<Result<(), String>>();
        self.running.store(true, Ordering::Relaxed);

        let running_clone = Arc::clone(&self.running);
        let config = self.config.clone();
        let logger_clone = self.logger.lock().ok().and_then(|guard| guard.clone());

        let handle = thread::spawn(move || {
            let rt = match Runtime::new() {
                Ok(rt) => rt,
                Err(e) => {
                    running_clone.store(false, Ordering::Relaxed);
                    let _ = tx.send(Err(format!("Failed to create Tokio runtime: {}", e)));
                    return;
                }
            };

            let ftp_home: PathBuf = PathBuf::from(&config.path);

            let users: Vec<UserInfo> = match serde_json::from_str(&config.users) {
                Ok(u) => u,
                Err(e) => {
                    running_clone.store(false, Ordering::Relaxed);
                    let _ = tx.send(Err(format!("Failed to parse users JSON: {}", e)));
                    return;
                }
            };

      let shutdown_running = Arc::clone(&running_clone);
      let logger_for_vfs = logger_clone.clone();
      let logger_for_auth = logger_clone.clone();
      
      let mut server_builder = libunftp::ServerBuilder::with_user_detail_provider(
        Box::new(move || {
          let vfs = Filesystem::new(ftp_home.clone())
            .expect("Failed to create filesystem - path should be valid and accessible");
          let restrict_vfs = unftp_sbe_restrict::RestrictingVfs::<Filesystem, UserInfo, Meta>::new(
            vfs,
          );
          
      // 包装 LoggingVfs 以记录所有文件操作
      let logging_vfs = crate::ftp::vfs_logger::create_logging_vfs(
        restrict_vfs,
        logger_for_vfs.clone()
      );
      logging_vfs
        }),
        std::sync::Arc::new(FtpUserDetailProvider {
          users: users.clone(),
          file_auth: config.file_auth.clone(),
        }),
      )
      .authenticator(std::sync::Arc::new(FtpUserAuthenticator {
        is_anonymous: config.is_anonymous,
        users: users.clone(),
        file_auth: config.file_auth.clone(),
        logger: logger_for_auth.clone(),
      }))
            .greeting("Welcome to my FTP server")
            .passive_ports(config.passive_port_range.clone())
            .shutdown_indicator(async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    if !shutdown_running.load(Ordering::Relaxed) {
                        break;
                    }
                }
                libunftp::options::Shutdown::new().grace_period(Duration::from_secs(2))
            });

            if let Some(ref logger_arc) = logger_clone {
                let listener = FtpDataListener::new(logger_arc.clone());
                server_builder = server_builder.notify_data(listener);
            }

            let new_server = match server_builder.build() {
                Ok(server) => server,
                Err(e) => {
                    running_clone.store(false, Ordering::Relaxed);
                    let _ = tx.send(Err(format!("Failed to build FTP server: {}", e)));
                    return;
                }
            };

            let _ = tx.send(Ok(()));

            let listen_result = rt.block_on(async { new_server.listen(bind_address_str).await });
            if let Err(e) = listen_result {
                eprintln!("FTP server error: {}", e);
            }

            running_clone.store(false, Ordering::Relaxed);
        });

        // 等待启动结果
        match rx.recv() {
            Ok(Ok(())) => {
                self.handle = Some(handle);
                Ok(())
            }
            Ok(Err(e)) => {
                self.running.store(false, Ordering::Relaxed);
                handle.join().map_err(|_| "FTP 服务线程未能正常结束")?;
                Err(e.into())
            }
            Err(_) => {
                self.running.store(false, Ordering::Relaxed);
                handle.join().map_err(|_| "FTP 服务线程未能正常结束")?;
                Err("Failed to receive startup result".into())
            }
        }
    }

    /// 停止 FTP 服务器
    ///
    /// 设置关闭标志并立即返回，不阻塞等待线程结束。
    /// 线程会在后台自行退出（最多约 3 秒），句柄在下次启动或 drop 时清理。
    pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.running.store(false, Ordering::Relaxed);
        // 不调用 handle.join()，让线程在后台自行退出
        // 句柄保留，由 reap_finished_handle 或 drop 清理
        Ok(())
    }

    /// 检查 FTP 服务器是否正在运行
    ///
    /// # 返回值
    /// * `true` - 服务器正在运行
    /// * `false` - 服务器已停止
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    fn reap_finished_handle(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self
            .handle
            .as_ref()
            .is_some_and(std::thread::JoinHandle::is_finished)
        {
            if let Some(handle) = self.handle.take() {
                handle.join().map_err(|_| "FTP 服务线程未能正常结束")?;
            }
            self.running.store(false, Ordering::Relaxed);
        }

        Ok(())
    }
}

fn bind_address(port: &str) -> Result<SocketAddr, Box<dyn std::error::Error>> {
    let port = port.parse::<u16>()?;
    Ok(SocketAddr::from(([0, 0, 0, 0], port)))
}

fn ensure_bind_available(bind_address: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(bind_address)?;
    drop(listener);
    Ok(())
}
