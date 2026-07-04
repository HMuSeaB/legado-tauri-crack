//! JNI 桥梁 — Rust 与 Java 之间的通信
//!
//! 从 .so 提取到 25 个 JNI 函数:
/// `Java_app_tauri_plugin_PluginManager_handlePluginResponse`
/// `Java_app_tauri_plugin_PluginManager_sendChannelData`
/// `Java_com_legado_1tauri_MainActivity_initRustlsPlatformVerifier`
/// `Java_com_legado_1tauri_Rust_assetLoaderDomain`
/// `Java_com_legado_1tauri_Rust_create`
/// `Java_com_legado_1tauri_Rust_handleReceivedTitle`
/// `Java_com_legado_1tauri_Rust_handleRequest`
/// `Java_com_legado_1tauri_Rust_ipc`
/// `Java_com_legado_1tauri_Rust_onActivityCreate`
/// `Java_com_legado_1tauri_Rust_onActivityDestroy`
/// `Java_com_legado_1tauri_Rust_onActivityLowMemory`
/// `Java_com_legado_1tauri_Rust_onActivitySaveInstanceState`
/// `Java_com_legado_1tauri_Rust_onEval`
/// `Java_com_legado_1tauri_Rust_onNewIntent`
/// `Java_com_legado_1tauri_Rust_onPageLoaded`
/// `Java_com_legado_1tauri_Rust_onPageLoading`
/// `Java_com_legado_1tauri_Rust_onWebviewDestroy`
/// `Java_com_legado_1tauri_Rust_onWindowFocusChanged`
/// `Java_com_legado_1tauri_Rust_pause`
/// `Java_com_legado_1tauri_Rust_resume`
/// `Java_com_legado_1tauri_Rust_shouldOverride`
/// `Java_com_legado_1tauri_Rust_start`
/// `Java_com_legado_1tauri_Rust_stop`
/// `Java_com_legado_1tauri_Rust_withAssetLoader`
/// `Java_com_legado_1tauri_Rust_wryCreate`

use jni::{JNIEnv, objects::JClass, objects::JString, objects::JObject};

// JNI 导出函数 (Rust 侧定义，Java 侧通过 System.loadLibrary 调用)
// 函数名格式: Java_com_legado_1tauri_ClassName_methodName
//
// 已确认的 Java 类:
// - com.legado_tauri.Rust            — 主 Rust 入口
// - com.legado_tauri.BrowserProbeHost — 浏览器探针
// - com.legado_tauri.MainActivity     — 主 Activity
// - app.tauri.plugin.PluginManager    — Tauri 插件管理

#[no_mangle]
pub extern "system" fn Java_app_tauri_plugin_PluginManager_handlePluginResponse(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_app_tauri_plugin_PluginManager_sendChannelData(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_MainActivity_initRustlsPlatformVerifier(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_assetLoaderDomain(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_create(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_handleReceivedTitle(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_handleRequest(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_ipc(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_onActivityCreate(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_onActivityDestroy(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_onActivityLowMemory(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_onActivitySaveInstanceState(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_onEval(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_onNewIntent(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_onPageLoaded(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_onPageLoading(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_onWebviewDestroy(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_onWindowFocusChanged(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_pause(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_resume(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_shouldOverride(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_start(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_stop(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_withAssetLoader(_env: JNIEnv, _class: JClass) { /* ... */ }
#[no_mangle]
pub extern "system" fn Java_com_legado_1tauri_Rust_wryCreate(_env: JNIEnv, _class: JClass) { /* ... */ }
