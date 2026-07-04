//! 同步模块
//!
//! 支持的同步方式: WebDAV, 百度网盘
//!
//! 命令 (25 个):
//! - sync_arrow_function
//! - sync_await
//! - sync_baidu_app_namedevice_uuid
//! - sync_baidu_app_namedevice_uuidstruct
//! - sync_baidu_poll_tokendevice
//! - sync_client
//! - sync_conflict_remote_
//! - sync_dependencies
//! - sync_eval_index
//! - sync_from_sync_iterator
//! - sync_function
//! - sync_function_expression
//! - sync_generator
//! - sync_generator_expression
//! - sync_generator_function
//! - sync_handler
//! - sync_impl
//! - sync_iterator
//! - sync_mobile_pause_on_low_batteryapplication
//! - sync_nowmin
//! - sync_parent_modulesimport_meta
//! - sync_read
//! - sync_runtime
//! - sync_state
//! - sync_trigger_on_bookshelf_change

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncCredentials {
    pub provider: String,
    pub profile_id: String,
    pub webdav_url: Option<String>,
    pub webdav_username: Option<String>,
    pub webdav_root_dir: Option<String>,
    pub webdav_allow_http: bool,
    pub baidu_refresh_token: Option<String>,
    pub baidu_token_expires_at: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflict {
    pub conflict_id: String,
    pub domain: String,
    pub key: String,
    pub local: String,
    pub remote: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
}
