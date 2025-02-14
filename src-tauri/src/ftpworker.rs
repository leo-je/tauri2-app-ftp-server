use std::{
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};
use tokio::runtime::Runtime;
use unftp_sbe_fs::ServerExt;
static TASK_RUNNING: AtomicBool = AtomicBool::new(false);

pub struct FtpWorker {
    pub path: String,
    pub port: String,
    pub handle: Option<thread::JoinHandle<()>>,
}
impl FtpWorker {
    pub fn new() -> Self {
        FtpWorker {
            handle: None,
            path: "".to_string(),
            port: "".to_string(),
        }
    }

    pub fn set(&mut self, _path: String, _port: String) {
        self.path = _path;
        self.port = _port;
    }

    pub fn start(&mut self) {
        if self.handle.is_none() {
            let p = self.path.clone();
            let p2 = self.port.clone();
            let handle = thread::spawn(move || {
                println!("thread start");
                // 在子线程中创建一个 Tokio 运行时
                let rt = Runtime::new().unwrap();

                // 在 Tokio 运行时的上下文中执行异步任务
                rt.block_on(async {
                    println!("Before calling async method");
                    let ftp_home: PathBuf = PathBuf::from(p);
                    println!("start_ftp_server-1");
                    let new_server = libunftp::Server::with_fs(ftp_home)
                        .greeting("Welcome to my FTP server")
                        .passive_ports(50000..65535)
                        .shutdown_indicator(async {
                            println!("shutdown_indicator");
                            loop {
                                println!("wait 1s");
                                tokio::time::sleep(Duration::from_secs(1)).await;
                                // Shut the server down after 10 seconds.
                                let b = is_running();
                                if !b {
                                    println!("not running");
                                    break;
                                }
                            }
                            libunftp::options::Shutdown::new().grace_period(Duration::from_secs(2))
                            // Allow 5 seconds to shutdown gracefully
                        })
                        .build()
                        .unwrap();
                    let _result = new_server.listen("0.0.0.0:".to_string() + &p2).await;
                    if let Err(e) = _result {
                        println!("{}", e.to_string())
                    }
                    println!("After calling async method");
                });
                println!("thread end");
            });
            self.handle = Some(handle);
            TASK_RUNNING.store(true, Ordering::Relaxed);
        }
    }

    pub fn stop(&mut self) {
        println!("stop");
        TASK_RUNNING.store(false, Ordering::Relaxed);
        self.handle = None
    }

    pub fn is_running(&mut self) -> bool {
        let b = TASK_RUNNING.load(Ordering::Relaxed);
        println!("is running:{}", b);
        return b;
    }
}

fn is_running() -> bool {
    let b = TASK_RUNNING.load(Ordering::Relaxed);
    println!("is running:{}", b);
    return b;
}
