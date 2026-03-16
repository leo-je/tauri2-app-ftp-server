//! 应用初始化命令
//!
//! 包含初始化步骤执行和状态查询的 Tauri 命令。

use serde::{Deserialize, Serialize};

/// 初始化状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitStatus {
    pub step: String,
    pub progress: u8,
    pub message: String,
    pub status: String, // "pending", "running", "completed", "error"
}

/// 初始化检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitCheckResult {
    pub success: bool,
    pub step: String,
    pub progress: u8,
    pub message: String,
    pub message_key: String,
    pub message_params: Vec<String>,
    pub can_continue: bool,
    pub estimated_time_ms: Option<u32>,
}

/// 执行初始化检查步骤
///
/// 根据步骤名称执行相应的初始化检查
#[tauri::command]
pub async fn run_init_step(
    step: String,
    app_handle: tauri::AppHandle,
) -> Result<InitCheckResult, String> {
    match step.as_str() {
        "system_check" => {
            // 随机睡眠 500-800ms
            let sleep_ms = rand::random::<u64>() % 301 + 500;
            tokio::time::sleep(tokio::time::Duration::from_millis(sleep_ms)).await;

            let sys_info = crate::commands::system::get_system_info()?;

            Ok(InitCheckResult {
                success: true,
                step: "system_check".to_string(),
                progress: 25,
                message: format!("{} {}", sys_info.platform, sys_info.arch),
                message_key: "splash.messages.systemCheckComplete".to_string(),
                message_params: vec![sys_info.platform, sys_info.arch],
                can_continue: true,
                estimated_time_ms: None,
            })
        }
        "config_load" => {
            // 随机睡眠 300-500ms
            let sleep_ms = rand::random::<u64>() % 201 + 300;
            tokio::time::sleep(tokio::time::Duration::from_millis(sleep_ms)).await;

            let config = crate::commands::system::check_app_config(app_handle)?;

            let (message_key, message) = if config.config_exists {
                ("splash.messages.configLoaded", "Config loaded".to_string())
            } else {
                ("splash.messages.usingDefaultConfig", "Using default config".to_string())
            };

            Ok(InitCheckResult {
                success: true,
                step: "config_load".to_string(),
                progress: 50,
                message,
                message_key: message_key.to_string(),
                message_params: vec![],
                can_continue: true,
                estimated_time_ms: None,
            })
        }
        "service_init" => {
            // 随机睡眠 800-1200ms
            let sleep_ms = rand::random::<u64>() % 401 + 800;
            tokio::time::sleep(tokio::time::Duration::from_millis(sleep_ms)).await;

            // 检查网络接口
            let interfaces = crate::commands::network::get_network_interfaces()?;
            let interface_count = interfaces.len().to_string();

            Ok(InitCheckResult {
                success: true,
                step: "service_init".to_string(),
                progress: 75,
                message: format!("Found {} interfaces", interface_count),
                message_key: "splash.messages.interfacesFound".to_string(),
                message_params: vec![interface_count],
                can_continue: true,
                estimated_time_ms: None,
            })
        }
        "ready" => {
            // 随机睡眠 200-300ms
            let sleep_ms = rand::random::<u64>() % 101 + 200;
            tokio::time::sleep(tokio::time::Duration::from_millis(sleep_ms)).await;

            Ok(InitCheckResult {
                success: true,
                step: "ready".to_string(),
                progress: 100,
                message: "System ready".to_string(),
                message_key: "splash.messages.systemReady".to_string(),
                message_params: vec![],
                can_continue: true,
                estimated_time_ms: None,
            })
        }
        _ => Err(format!("未知的初始化步骤: {}", step)),
    }
}

/// 获取完整的初始化状态
#[tauri::command]
pub fn get_init_status() -> Vec<InitStatus> {
    vec![
        InitStatus {
            step: "system_check".to_string(),
            progress: 0,
            message: "等待系统检测".to_string(),
            status: "pending".to_string(),
        },
        InitStatus {
            step: "config_load".to_string(),
            progress: 0,
            message: "等待配置加载".to_string(),
            status: "pending".to_string(),
        },
        InitStatus {
            step: "service_init".to_string(),
            progress: 0,
            message: "等待服务初始化".to_string(),
            status: "pending".to_string(),
        },
        InitStatus {
            step: "ready".to_string(),
            progress: 0,
            message: "等待准备就绪".to_string(),
            status: "pending".to_string(),
        },
    ]
}
