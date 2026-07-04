//! 状态管理 — 每个 probe 实例的 session/cookie/UA 状态
//!
//! 从二进制提取的相关符号:
//! - Android: BrowserProbeHost.
//! - Android: BrowserProbeHost.cookies
//! - Android: BrowserProbeHost.eval
//! - Android: BrowserProbeHost.waitMessage
//! - BrowserProbeHost.
//! - BrowserProbeHost.cookies
//! - BrowserProbeHost.create
//! - BrowserProbeHost.currentUrl
//! - BrowserProbeHost.eval
//! - BrowserProbeHost.navigate
//! - BrowserProbeHost.onRequest
//! - BrowserProbeHost.respondMessage
//! - BrowserProbeHost.setCookie
//! - BrowserProbeHost.setMuted
//! - BrowserProbeHost.waitMessage

use serde::{Serialize, Deserialize};

/// BrowserProbe 会话状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserProbeSession {
    pub session_id: String,
    pub user_agent: Option<String>,
    pub cookies: Vec<BrowserCookie>,
    pub visible: bool,
    pub muted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserCookie {
    pub domain: String,
    pub key: String,
    pub value: String,
    pub expires: Option<u64>,
    pub secure: bool,
}

/// 前端 HTTP 请求 (由前端发起，通过 Rust 代理)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendPluginHttpRequest {
    pub url: String,
    pub method: String,
    pub headers: Option<std::collections::HashMap<String, String>>,
    pub timeout_secs: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendPluginHttpResponse {
    pub status: u16,
    pub headers: Option<std::collections::HashMap<String, String>>,
    pub body: String,
}
