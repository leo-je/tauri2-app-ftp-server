use std::{
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread,
    time::Duration,
};
use tokio::runtime::Runtime;
use unftp_sbe_fs::ServerExt;

pub struct FtpWorker {
    pub path: String,
    pub port: String,
    pub handle: Option<thread::JoinHandle<()>>,
    running: Arc<AtomicBool>,
}

impl FtpWorker {
    pub fn new() -> Self {
        let running = Arc::new(AtomicBool::new(false));
        FtpWorker {
            handle: None,
            path: "/default/path".to_string(), // 提供默认路径
            port: "2121".to_string(),          // 提供默认端口
            running,
        }
    }

    pub fn set(&mut self, path: String, port: String) {
        self.path = path;
        self.port = port;
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.handle.is_none() {
            let path = self.path.clone();
            let port = self.port.clone();
            let running_clone = Arc::clone(&self.running);

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
                    let ftp_home: PathBuf = PathBuf::from(path);
                    println!("start_ftp_server-1");

                    let new_server = match libunftp::Server::with_fs(ftp_home)
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

                    match new_server.listen(format!("0.0.0.0:{}", port)).await {
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

    pub fn stop(&mut self) {
        println!("stop");
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            handle.join().expect("Thread failed to join");
        }
    }

    pub fn is_running(&self) -> bool {
        let b = self.running.load(Ordering::Relaxed);
        // println!("is running:{}", b); // 移除不必要的日志输出
        b
    }
}