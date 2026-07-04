//! 应用配置 (AppConfig)
//!
//! 从 .so 提取到 14 个配置项:
//! - booksource_dirs
//! - booksource_get_dirbooksource_get_dirsbooksource_listbooksource_list_streamingbooksource_readbooksource_savebooksource_deletebooksource_save_draftbooksource_delete_draftbooksource_togglebooksource_open_in_vscodebooksource_resolve_pathbooksource_evalbooksource_add_dirbooksource_remove_dirbooksource_pick_dir
//! - browser_probe_tauri
//! - browser_probe_visible_by_defaultg
//! - comic_cache_sizeextension_toggleacceptFirstMouseLgTauri
//! - request_id
//! - request_parts
//! - sync_mobile_pause_on_low_batteryapplication
//! - sync_nowminWidthreuseKeymicaDark
//! - sync_state
//! - sync_trigger_on_bookshelf_change
//! - tts_rateapp
//! - video_proxy
//! - web_server_enabled

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub booksource_dirs: Option<String>,
    pub booksource_get_dirbooksource_get_dirsbooksource_listbooksource_list_streamingbooksource_readbooksource_savebooksource_deletebooksource_save_draftbooksource_delete_draftbooksource_togglebooksource_open_in_vscodebooksource_resolve_pathbooksource_evalbooksource_add_dirbooksource_remove_dirbooksource_pick_dir: Option<String>,
    pub browser_probe_tauri: Option<String>,
    pub browser_probe_visible_by_defaultg: Option<String>,
    pub comic_cache_sizeextension_toggleacceptFirstMouseLgTauri: Option<String>,
    pub request_id: Option<String>,
    pub request_parts: Option<String>,
    pub sync_mobile_pause_on_low_batteryapplication: Option<String>,
    pub sync_nowminWidthreuseKeymicaDark: Option<String>,
    pub sync_state: Option<String>,
    pub sync_trigger_on_bookshelf_change: Option<String>,
    pub tts_rateapp: Option<String>,
    pub video_proxy: Option<String>,
    pub web_server_enabled: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            booksource_dirs: None,
            booksource_get_dirbooksource_get_dirsbooksource_listbooksource_list_streamingbooksource_readbooksource_savebooksource_deletebooksource_save_draftbooksource_delete_draftbooksource_togglebooksource_open_in_vscodebooksource_resolve_pathbooksource_evalbooksource_add_dirbooksource_remove_dirbooksource_pick_dir: None,
            browser_probe_tauri: None,
            browser_probe_visible_by_defaultg: None,
            comic_cache_sizeextension_toggleacceptFirstMouseLgTauri: None,
            request_id: None,
            request_parts: None,
            sync_mobile_pause_on_low_batteryapplication: None,
            sync_nowminWidthreuseKeymicaDark: None,
            sync_state: None,
            sync_trigger_on_bookshelf_change: None,
            tts_rateapp: None,
            video_proxy: None,
            web_server_enabled: None,
        }
    }
}
