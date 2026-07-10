//! Tauri IPC 命令注册中心
//! 从 .so 提取到 210 个命令

use std::sync::OnceLock;
pub static GLOBAL_APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();

pub mod booksource;
pub mod browser_probe;
pub mod comic;
pub mod cache;
pub mod extension;
pub mod script;
pub mod config;
pub mod frontend;
pub mod sync;
pub mod bookshelf;
pub mod tts;
pub mod web_server;
pub mod system;

pub fn generate_handler() -> impl Fn(tauri::ipc::Invoke<tauri::Wry>) -> bool + Send + Sync + 'static {
    |invoke| {
        let cmd = invoke.message.command();
        match cmd {
            "extension_list" => {
                invoke.resolver.resolve(serde_json::json!([]));
                true
            }
            "extension_get_dir" => {
                invoke.resolver.resolve(serde_json::json!(""));
                true
            }
            "app_config_get_all" => {
                let default_cfg: serde_json::Value = serde_json::from_str(r##"{
                    "http_user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
                    "http_follow_redirects": true,
                    "http_connect_timeout_secs": 10,
                    "http_ignore_tls_errors": true,
                    "http_doh_server": "none",
                    "proxy_mode": "system",
                    "proxy_type": "http",
                    "proxy_host": "",
                    "proxy_port": 0,
                    "proxy_username": "",
                    "proxy_password": "",
                    "engine_timeout_secs": 30,
                    "booksource_watcher_enabled": false,
                    "browser_probe_enabled": true,
                    "browser_probe_user_agent": "",
                    "browser_probe_timeout_secs": 0,
                    "browser_probe_visible_by_default": false,
                    "browser_probe_force_visible": false,
                    "browser_probe_persist_profile": true,
                    "comic_cache_enabled": true,
                    "ui_layout_mode": "auto",
                    "ui_theme": "auto",
                    "ui_enable_aplus_tracking": true,
                    "power_keep_awake_on_tts": false,
                    "power_reader_awake_mode": "off",
                    "power_reader_awake_timeout_secs": 600,
                    "windows_main_window_width": 0,
                    "windows_main_window_height": 0,
                    "video_player_type": "xgplayer",
                    "video_default_rate": 1.0,
                    "video_auto_next": true,
                    "video_quality_prefer": "auto",
                    "video_remember_progress": true,
                    "video_seek_step_secs": 10,
                    "video_vjs_preload": "auto",
                    "video_vjs_pip": true,
                    "video_xg_download": false,
                    "video_dp_danmaku": false,
                    "video_dp_theme": "#00b1ff",
                    "video_autoplay": true,
                    "web_server_enabled": false,
                    "web_server_port": 7688,
                    "web_server_dist_path": "",
                    "web_remote_debug_enabled": false,
                    "web_remote_debug_host": "",
                    "web_remote_debug_port": 8080,
                    "request_min_delay_ms": 300,
                    "cache_prefetch_count": 3,
                    "cache_prefetch_concurrency": 2,
                    "export_prefetch_concurrency": 3,
                    "sync_enabled": false,
                    "sync_provider": "webdav",
                    "sync_profile_id": "default",
                    "sync_webdav_url": "",
                    "sync_webdav_username": "",
                    "sync_webdav_root_dir": "legado-sync",
                    "sync_webdav_allow_http": false,
                    "sync_trigger_enabled": true,
                    "sync_timer_enabled": false,
                    "sync_timer_interval_secs": 900,
                    "sync_trigger_on_startup": true,
                    "sync_trigger_on_resume": true,
                    "sync_trigger_on_unlock_resume": true,
                    "sync_trigger_on_bookshelf_change": false,
                    "sync_trigger_on_booksource_change": false,
                    "sync_trigger_on_settings_change": false,
                    "sync_scope_bookshelf": true,
                    "sync_scope_reading_progress": true,
                    "sync_scope_booksources": true,
                    "sync_scope_reader_settings": true,
                    "sync_scope_app_settings": true,
                    "sync_scope_source_flags": false,
                    "sync_scope_extensions": false,
                    "sync_scope_script_config": false,
                    "sync_mobile_foreground_only": true,
                    "sync_mobile_screen_on_only": true,
                    "sync_mobile_wifi_only": true,
                    "sync_mobile_pause_on_low_battery": true,
                    "sync_mobile_startup_delay_ms": 5000,
                    "sync_mobile_resume_delay_ms": 1500,
                    "sync_baidu_app_name": "legado-tauri"
                }"##).unwrap();
                invoke.resolver.resolve(default_cfg);
                true
            }
            "bookshelf_list" => {
                invoke.resolver.resolve(serde_json::json!([]));
                true
            }
            "booksource_list" | "booksource_get_all" => {
                if let Some(app) = GLOBAL_APP_HANDLE.get() {
                    let app = app.clone();
                    tauri::async_runtime::spawn(async move {
                        match booksource::real_booksource_list(&app).await {
                            Ok(res) => invoke.resolver.resolve(res),
                            Err(err) => invoke.resolver.reject(err.to_string()),
                        }
                    });
                    true
                } else {
                    invoke.resolver.reject("App handle not initialized".to_string());
                    true
                }
            }
            "booksource_import_legacy_json_url" => {
                if let Some(app) = GLOBAL_APP_HANDLE.get() {
                    let app = app.clone();
                    let payload = invoke.message.payload().as_json();
                    let url = payload.and_then(|p| p.get("url")).and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let smart_explore = payload.and_then(|p| p.get("smartExploreSubCategories")).and_then(|v| v.as_bool()).unwrap_or(false);
                    
                    tauri::async_runtime::spawn(async move {
                        match booksource::real_import_legacy_json_url(&app, &url, smart_explore).await {
                            Ok(res) => invoke.resolver.resolve(res),
                            Err(err) => invoke.resolver.reject(err.to_string()),
                        }
                    });
                    true
                } else {
                    invoke.resolver.reject("App handle not initialized".to_string());
                    true
                }
            }
            "booksource_import_legacy_json_text" => {
                if let Some(app) = GLOBAL_APP_HANDLE.get() {
                    let app = app.clone();
                    let payload = invoke.message.payload().as_json();
                    let content = payload.and_then(|p| p.get("content")).and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let smart_explore = payload.and_then(|p| p.get("smartExploreSubCategories")).and_then(|v| v.as_bool()).unwrap_or(false);
                    
                    tauri::async_runtime::spawn(async move {
                        match booksource::real_import_legacy_json_text(&app, &content, smart_explore).await {
                            Ok(res) => invoke.resolver.resolve(res),
                            Err(err) => invoke.resolver.reject(err.to_string()),
                        }
                    });
                    true
                } else {
                    invoke.resolver.reject("App handle not initialized".to_string());
                    true
                }
            }
            "list_user_fonts" => {
                invoke.resolver.resolve(serde_json::json!([]));
                true
            }
            "get_local_ips" => {
                invoke.resolver.resolve(serde_json::json!([]));
                true
            }
            "sync_get_status" => {
                invoke.resolver.resolve(serde_json::json!({
                    "status": "idle"
                }));
                true
            }
            _ => {
                if cmd.starts_with("app_config_") 
                    || cmd.starts_with("bookshelf_") 
                    || cmd.starts_with("booksource_")
                    || cmd.starts_with("extension_")
                    || cmd.starts_with("sync_")
                    || cmd.starts_with("web_server_")
                    || cmd.starts_with("frontend_")
                    || cmd.starts_with("audio_")
                    || cmd.starts_with("cover_")
                    || cmd.starts_with("tts_")
                {
                    invoke.resolver.resolve(serde_json::json!([]));
                    true
                } else {
                    false
                }
            }
        }
    }
}

/// 全部命令名 (从 .so 二进制提取)
pub const ALL_COMMANDS: &[&str] = &[
    "absoluteis_absolutenormalizeresolve_directorydeny",
    "activity_nameplugin",
    "app_config_resetweb_server_startdeny",
    "attentionrequest_user_attentionscale_factorallow",
    "audio_cache",
    "audio_cache_sizeallow",
    "bookshelf_change",
    "bookshelf_export_booksync_baidu_poll_tokendevice",
    "bookshelf_get_episode_progresswss",
    "bookshelf_meta",
    "bookshelf_pick_save_pathdefault",
    "bookshelf_removeread",
    "booksource_dirs",
    "booksource_eval",
    "booksource_pick_dir",
    "booksource_remove_dirbooksource_pick_dir",
    "booksource_test",
    "bottomset_always_on_bottomallow",
    "browser_probe_on_requestcapture",
    "browser_probe_page_messagemessagesession",
    "browser_probe_profileeventscript",
    "browser_probe_tauri",
    "browser_probe_visible_by_defaultg",
    "bundle_typelog",
    "closableset_closableallow",
    "closewebview_closeallow",
    "colorset_background_colorallow",
    "colorset_webview_background_colorallow",
    "comic_cache",
    "comic_cache_sizeextension_toggleaccept",
    "config_create_navigate_eval_html_texturl_set",
    "config_idkem_idpublic_keysymmetric_cipher_suites",
    "config_read_jsondelete_user_fontdeny",
    "config_resetweb_server_startdeny",
    "constraintsset_size_constraintsallow",
    "copy_fileplugin",
    "countset_badge_countallow",
    "cover_cache",
    "cover_cache_sizebooksource",
    "cover_cachedata",
    "create_defaultplugin",
    "create_webview_windowplugin",
    "createcurrent_monitorcursor_positionallow",
    "current_monitorplugin",
    "currentget_currentallow",
    "cursor_positionplugin",
    "dataclear_all_browsing_dataallow",
    "debug_types",
    "decorationsset_decorationsallow",
    "destroyget_all_windowsallow",
    "draggingstart_draggingallow",
    "draggingstart_resize_draggingallow",
    "effectsset_effectsallow",
    "elementsdevice_codeuser_codestruct",
    "emit_toplugin",
    "eventsset_ignore_cursor_eventsallow",
    "extension_toggleaccept",
    "filecopy_fileallow",
    "fileread_fileallow",
    "fileread_text_fileallow",
    "filewrite_text_filescope",
    "focusableset_focusableallow",
    "focusset_focusallow",
    "focusset_webview_focusallow",
    "from_bytesplugin",
    "from_pathplugin",
    "frontend_plugin_http_requestcore",
    "frontend_plugin_http_requesttts_synthesize",
    "frontend_storage_list",
    "frontend_storage_list_namespacesallow",
    "frontend_storage_removefrontend_storage_list",
    "fullscreenset_fullscreenallow",
    "fullscreenset_simple_fullscreenallow",
    "get_all_webviewsplugin",
    "get_all_windowsplugin",
    "get_by_idplugin",
    "get_currentplugin",
    "grabset_cursor_graballow",
    "hidewebview_hideallow",
    "icondefault_window_iconallow",
    "iconset_cursor_iconallow",
    "inner_positionplugin",
    "inner_sizeplugin",
    "internal_toggle_devtoolsplugin",
    "internal_toggle_maximizeplugin",
    "is_absoluteplugin",
    "is_always_on_topplugin",
    "is_checkedplugin",
    "is_closableplugin",
    "is_decoratedplugin",
    "is_enabledplugin",
    "is_focusedplugin",
    "is_fullscreenplugin",
    "is_maximizableplugin",
    "is_maximizedplugin",
    "is_minimizableplugin",
    "is_minimizedplugin",
    "is_resizableplugin",
    "is_visibleplugin",
    "labelset_badge_labelallow",
    "linkget_machine_uidcom",
    "machine_uidstream",
    "maximizableset_maximizableallow",
    "maximizeactivity_nameavailable_monitorsallow",
    "maximizetoggle_maximizeunmaximizeunminimizedeny",
    "minimizableset_minimizableallow",
    "open_pathplugin",
    "open_urlplugin",
    "outer_positionplugin",
    "outer_sizeplugin",
    "pathopen_pathreveal_item_in_dirdeny",
    "positionset_cursor_positionallow",
    "positionset_positionallow",
    "positionset_webview_positionallow",
    "primary_monitorplugin",
    "protectedset_content_protectedallow",
    "read_dirplugin",
    "read_fileplugin",
    "read_text_file_lines_nextplugin",
    "read_text_file_linesplugin",
    "read_text_fileplugin",
    "register_listenerplugin",
    "registeredis_registeredallow",
    "remove_atplugin",
    "remove_by_idplugin",
    "remove_listenerplugin",
    "rename_user_fontreading",
    "resizableset_resizableset_shadowallow",
    "resizeset_webview_auto_resizeallow",
    "resolve_directoryplugin",
    "scale_factorplugin",
    "scene_identifierplugin",
    "script__",
    "script____",
    "script_bridge",
    "set_acceleratorplugin",
    "set_as_app_menuplugin",
    "set_as_help_menu_for_nsappplugin",
    "set_as_window_menuplugin",
    "set_as_windows_menu_for_nsappplugin",
    "set_checkedplugin",
    "set_enabledplugin",
    "set_focusplugin",
    "set_icon_with_as_templateplugin",
    "set_iconplugin",
    "set_menuplugin",
    "set_show_menu_on_left_clickplugin",
    "set_temp_dir_pathplugin",
    "set_textplugin",
    "set_titleplugin",
    "set_tooltipplugin",
    "set_visibleplugin",
    "showbundle_typeallow",
    "showwebview_showwebview_sizedeny",
    "sizefrom_bytesfrom_pathdeny",
    "sizeset_max_sizeallow",
    "sizeset_min_sizeallow",
    "stackfrontend_storage_list_namespacesallow",
    "start_draggingplugin",
    "storage_list",
    "storage_list_namespacesallow",
    "storage_removefrontend_storage_list",
    "storeremove_data_storeremove_listenerallow",
    "styleset_title_bar_styleallow",
    "supports_multiple_windowsplugin",
    "sync_arrow_function",
    "sync_await",
    "sync_baidu_app_namedevice_uuid",
    "sync_baidu_app_namedevice_uuidstruct",
    "sync_baidu_poll_tokendevice",
    "sync_client",
    "sync_conflict_remote_",
    "sync_dependencies",
    "sync_eval_index",
    "sync_from_sync_iterator",
    "sync_function",
    "sync_function_expression",
    "sync_generator",
    "sync_generator_expression",
    "sync_generator_function",
    "sync_handler",
    "sync_impl",
    "sync_iterator",
    "sync_mobile_pause_on_low_batteryapplication",
    "sync_nowmin",
    "sync_parent_modulesimport_meta",
    "sync_read",
    "sync_runtime",
    "sync_state",
    "sync_trigger_on_bookshelf_change",
    "tauri_versionplugin",
    "themeset_app_themeallow",
    "themeset_themeallow",
    "toggle_maximize",
    "topset_always_on_topallow",
    "tts_rateapp",
    "tts_synthesize",
    "tts_synthesizevoicevolumepitchdomainsappwss",
    "uidget_machine_uiddeny",
    "visibleset_cursor_visibleallow",
    "web_server_enabled",
    "web_server_startdeny",
    "webdavbaidu_netdiskscript",
    "webview_positionplugin",
    "webview_sizeplugin",
    "webviewcreate_webviewallow",
    "workspacesset_visible_on_all_workspacesallow",
    "write_fileplugin",
    "write_text_fileplugin",
    "writewrite_fileallow",
];
