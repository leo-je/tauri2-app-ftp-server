//! FTP 工作线程模块
//!
//! 该模块负责管理 FTP 服务器的生命周期，包括启动和停止 FTP 服务。
//! 使用独立的线程运行 FTP 服务器，避免阻塞主线程。

use crate::ftp::{ftp_user_authenticator::FtpUserAuthenticator, ftpuser::UserInfo};
use std::{
    ops::Range,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc,
        Arc,
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
    pub passive_port_range: Range<u16>,
}

impl Default for FtpWorkerConfig {
    fn default() -> Self {
        Self {
            path: "/default/path".to_string(),
            port: "2121".to_string(),
            users: "".to_string(),
            is_anonymous: true,
            file_auth: "R".to_string(),
            passive_port_range: DEFAULT_PASSIVE_PORT_START..DEFAULT_PASSIVE_PORT_END,
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
}

impl Default for FtpWorker {
    /// 创建默认的 FTP 工作线程实例
    fn default() -> Self {
        Self::new()
    }
}

impl FtpWorker {
    /// 创建新的 FTP 工作线程实例
    ///
    /// # 返回值
    /// 返回初始化的 FtpWorker 实例，默认配置为：
    /// - 路径："/default/path"
    /// - 端口："2121"
    /// - 匿名访问：启用
    /// - 权限：只读（"R"）
    pub fn new() -> Self {
        let running = Arc::new(AtomicBool::new(false));
        FtpWorker {
            handle: None,
            config: FtpWorkerConfig::default(),
            running,
        }
    }

    /// 设置 FTP 服务器配置
    ///
    /// # 参数
    /// * `config` - FTP 服务器配置
    pub fn set(&mut self, config: FtpWorkerConfig) {
        self.config = config;
    }

    /// 启动 FTP 服务器
    ///
    /// 在新线程中启动 FTP 服务器，使用 Tokio 运行时处理异步操作。
    /// 如果服务器已经在运行，则不会重复启动。
    ///
    /// # 返回值
    /// * `Ok(())` - 启动成功
    /// * `Err(...)` - 启动失败，返回错误信息
    ///
    /// # 注意事项
    /// - 默认使用被动模式端口范围 50000-65535（可通过 `passive_port_range` 配置）
    /// - 支持优雅关闭，关闭时等待 2 秒
    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_running() || self.handle.is_some() {
            return Ok(());
        }

        let (tx, rx) = mpsc::channel::<Result<(), String>>();
        let running_clone = Arc::clone(&self.running);
        let config = self.config.clone();

        let handle = thread::spawn(move || {
            println!("thread start");

            let rt = match Runtime::new() {
                Ok(rt) => rt,
                Err(e) => {
                    let _ = tx.send(Err(format!("Failed to create Tokio runtime: {}", e)));
                    return;
                }
            };

            rt.block_on(async {
                println!("Before calling async method");
                let ftp_home: PathBuf = PathBuf::from(&config.path);
                println!("start_ftp_server-1");

                let users: Vec<UserInfo> = match serde_json::from_str(&config.users) {
                    Ok(u) => u,
                    Err(e) => {
                        let _ = tx.send(Err(format!("Failed to parse users JSON: {}", e)));
                        return;
                    }
                };

                let server_builder = libunftp::ServerBuilder::with_authenticator(
                    Box::new(move || {
                        unftp_sbe_restrict::RestrictingVfs::<Filesystem, UserInfo, Meta>::new(
                            Filesystem::new(ftp_home.clone()),
                        )
                    }),
                    std::sync::Arc::new(FtpUserAuthenticator {
                        is_anonymous: config.is_anonymous,
                        users,
                        file_auth: config.file_auth,
                    }),
                )
                .greeting("Welcome to my FTP server")
                .passive_ports(config.passive_port_range.clone())
                .shutdown_indicator(async move {
                    loop {
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        if !running_clone.load(Ordering::Relaxed) {
                            break;
                        }
                    }
                    libunftp::options::Shutdown::new().grace_period(Duration::from_secs(2))
                });

                let new_server = match server_builder.build() {
                    Ok(server) => server,
                    Err(e) => {
                        let _ = tx.send(Err(format!("Failed to build FTP server: {}", e)));
                        return;
                    }
                };

                // 通知主线程服务器启动成功
                let _ = tx.send(Ok(()));

                // 启动 FTP 服务监听
                match new_server.listen(format!("0.0.0.0:{}", config.port)).await {
                    Ok(_) => println!("FTP server stopped successfully"),
                    Err(e) => eprintln!("FTP server error: {}", e),
                }
                println!("After calling async method");
            });
            println!("thread end");
        });

        // 等待启动结果
        match rx.recv() {
            Ok(Ok(())) => {
                self.handle = Some(handle);
                self.running.store(true, Ordering::Relaxed);
                Ok(())
            }
            Ok(Err(e)) => Err(e.into()),
            Err(_) => Err("Failed to receive startup result".into()),
        }
    }

    /// 停止 FTP 服务器
    ///
    /// 设置关闭标志并等待 FTP 服务线程结束
    ///
    /// # 返回值
    /// * `Ok(())` - 停止成功
    /// * `Err(...)` - 停止失败
    pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("stop");
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            handle.join().map_err(|_| "FTP 服务线程未能正常结束")?;
        }
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
}
