// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};
use tauri::Manager;
use tokio::runtime::Runtime;
use unftp_sbe_fs::ServerExt;
use tauri_plugin_log::{Target, TargetKind};

static TASK_RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
struct Worker {
    part: String,
    port: String,
    handle: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new() -> Self {
        Worker {
            handle: None,
            part: "".to_string(),
            port: "".to_string(),
        }
    }

    fn set(&mut self, _part: String, _port: String) {
        self.part = _part;
        self.port = _port;
    }

    fn start(&mut self) {
        if self.handle.is_none() {
            let p = self.part.clone();
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

    fn stop(&mut self) {
        println!("stop");
        TASK_RUNNING.store(false, Ordering::Relaxed);
        self.handle = None
    }
}

#[tauri::command]
fn is_running() -> bool {
    let b = TASK_RUNNING.load(Ordering::Relaxed);
    println!("is running:{}", b);
    return b;
}

#[tauri::command]
fn start_ftp_server(
    worker: tauri::State<'_, Arc<Mutex<Worker>>>,
    path: String,
    port: String,
) -> Result<String, String> {
    // 检查是否已经有子线程在运行
    let is_start: bool = is_running();
    if is_start {
        println!("Thread is already running");
        return Ok("服务已启动".to_string());
    }
    let p = path.clone();
    let p2 = port.clone();
    let mut worker = worker.lock().unwrap();
    worker.set(p, p2);
    worker.start();
    // 等待子线程完成
    // handle.join().unwrap();
    println!("start_ftp_server-3");
    Ok(format!("ok"))
}

#[tauri::command]
fn stop_ftp_server(worker: tauri::State<'_, Arc<Mutex<Worker>>>) -> Result<String, ()> {
    let mut worker = worker.lock().unwrap();
    worker.stop();
    Ok(format!("ok"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(move |app| {
            let stop_flag = Arc::new(AtomicBool::new(false));
            let worker = Arc::new(Mutex::new(Worker::new()));
            app.manage(stop_flag);
            app.manage(worker);
            Ok(())
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shellx::init(true))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            start_ftp_server,
            stop_ftp_server,
            is_running
        ])
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
