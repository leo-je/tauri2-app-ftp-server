//! Tauri 命令模块
//!
//! 包含前端可以调用的所有 Rust 命令函数，按职责拆分为子模块。

pub mod ftp;
pub mod init;
pub mod network;
pub mod system;
pub mod ip_blacklist;

// Re-export 所有 #[tauri::command] 函数，保持 lib.rs 中 invoke_handler 路径不变
pub use ftp::{start_ftp_server, stop_ftp_server};
pub use init::{run_init_step, get_init_status};
pub use network::{get_primary_ipv4, get_network_interfaces};
pub use system::{get_system_info, check_app_config, check_permissions, 
set_server_running, get_server_running};
