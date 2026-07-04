//! BrowserProbe Tauri 插件
//!
//! Android WebView 隐身浏览器探针，用于书源规则测试和网页交互。
//! 对应 Java 类: com.legado_tauri.BrowserProbeHost
//!
//! 已确认源文件 (从 .so 路径提取):
//! - src/browser_probe_tauri/bridge.rs     — JNI 桥梁
//! - src/browser_probe_tauri/commands.rs   — Tauri 命令
//! - src/browser_probe_tauri/create.rs      — WebView 创建
//! - src/browser_probe_tauri/lifecycle.rs   — 生命周期
//! - src/browser_probe_tauri/navigation.rs  — 导航和 JS 执行
//! - src/browser_probe_tauri/state.rs       — 状态管理
//!
//! BrowserProbeHost 方法:
///   - Android: BrowserProbeHost.
///   - Android: BrowserProbeHost.cookies
///   - Android: BrowserProbeHost.eval
///   - Android: BrowserProbeHost.waitMessage
///   - BrowserProbeHost.
///   - BrowserProbeHost.cookies
///   - BrowserProbeHost.create
///   - BrowserProbeHost.currentUrl
///   - BrowserProbeHost.eval
///   - BrowserProbeHost.navigate
///   - BrowserProbeHost.onRequest
///   - BrowserProbeHost.respondMessage
///   - BrowserProbeHost.setCookie
///   - BrowserProbeHost.setMuted
///   - BrowserProbeHost.waitMessage

pub mod bridge;
pub mod commands;
pub mod create;
pub mod lifecycle;
pub mod navigation;
pub mod state;
