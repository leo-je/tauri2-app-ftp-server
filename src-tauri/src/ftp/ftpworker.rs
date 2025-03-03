use crate::ftp::{ftp_user_authenticator::FtpUserAuthenticator, ftpuser::UserInfo};
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
use tokio::runtime::Runtime;
use unftp_sbe_fs::{Filesystem, Meta};

#[derive(Clone, Debug)]
pub struct FtpWorkerConfig {
    pub path: String,
    pub port: String,
    pub users: String,
    pub is_anonymous: bool,
    pub fileauth: String,
}

pub struct FtpWorker {
    pub config: FtpWorkerConfig,
    pub handle: Option<thread::JoinHandle<()>>,
    running: Arc<AtomicBool>,
}

impl FtpWorker {
    pub fn new() -> Self {
        let running = Arc::new(AtomicBool::new(false));
        FtpWorker {
            handle: None,
            config: FtpWorkerConfig {
                path: "/default/path".to_string(),
                port: "2121".to_owned(),
                users: "".to_string(),
                is_anonymous: true,
                fileauth: "R".to_string(),
            },
            running,
        }
    }

    pub fn set(&mut self, config: FtpWorkerConfig) {
        self.config = config;
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.handle.is_none() {
            let running_clone = Arc::clone(&self.running);
            let config = self.config.clone();
            // 创建一个线程
            let handle = thread::spawn(move || {
                println!("thread start");
                let rt = match Runtime::new() {
                    Ok(rt) => rt,
                    Err(e) => {
                        eprintln!("Failed to create Tokio runtime: {}", e);
                        return;
                    }
                };

                rt.block_on(async {
                    println!("Before calling async method");
                    let ftp_home: PathBuf = PathBuf::from(config.path);
                    println!("start_ftp_server-1");
                    // 将users转换为Vec<User>,users的格式为:[{"username":"admin","password":"111111"}]
                    let users: Vec<UserInfo> = serde_json::from_str(&config.users).unwrap();

                    let new_server = match libunftp::ServerBuilder::with_authenticator(
                        // Box::new(move || unftp_sbe_fs::Filesystem::new(ftp_home.clone())),
                        Box::new(move || {
                            unftp_sbe_restrict::RestrictingVfs::<Filesystem, UserInfo, Meta>::new(
                                Filesystem::new(ftp_home.clone()),
                            )
                        }),
                        std::sync::Arc::new(FtpUserAuthenticator {
                            is_anonymous: config.is_anonymous,
                            users,
                            fileauth: config.fileauth,
                        }),
                    )
                    .greeting("Welcome to my FTP server")
                    .passive_ports(50000..65535)
                    .shutdown_indicator(async move {
                        loop {
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            if !running_clone.load(Ordering::Relaxed) {
                                break;
                            }
                        }
                        libunftp::options::Shutdown::new().grace_period(Duration::from_secs(2))
                    })
                    .build()
                    {
                        Ok(server) => server,
                        Err(e) => {
                            eprintln!("Failed to build FTP server: {}", e);
                            return;
                        }
                    };

                    match new_server.listen(format!("0.0.0.0:{}", config.port)).await {
                        Ok(_) => println!("FTP server started successfully"),
                        Err(e) => eprintln!("Failed to start FTP server: {}", e),
                    }
                    println!("After calling async method");
                });
                println!("thread end");
            });

            self.handle = Some(handle);
            self.running.store(true, Ordering::Relaxed);
        }
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("stop");
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            handle.join().expect("Thread failed to join");
        }
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        let b = self.running.load(Ordering::Relaxed);
        // println!("is running:{}", b); // 移除不必要的日志输出
        b
    }
}
