//! legado_tauri_lib — Tauri Rust 核心库
//!
//! 架构 (从 liblegado_tauri_lib.so 逆向):
//! - JS 引擎: boa_ast-0.21.1, boa_engine-0.21.1, boa_gc-0.21.1, boa_interner-0.21.1, boa_parser-0.21.1, boa_string-0.21.1
//! - 框架: tauri-2.11.1, wry-0.55.1, tao-0.35.2
//! - 数据库: redb-4.1.0
//! - HTTP: reqwest-0.13.3, hyper-1.9.0, rustls-0.23.40
//! - HTML: scraper-0.27, dom_query-0.27, html5ever-0.39
//! - 缓存: moka-0.12.15
//! - 加密: ring-0.17.14, aes-0.9, sha2-0.11, hmac-0.13
//! - JNI: jni-0.22.4
//! - 图片: image-0.25.10, qrcode-0.14.1
//! - 压缩: flate2-1.1.9, zip-8.6.0
//! - 开发者: q3499

pub mod commands;
pub mod browser_probe_tauri;
pub mod engine;
pub mod http;
pub mod config;
pub mod sync;
pub mod bookshelf;
pub mod cache;
pub mod storage;
pub mod repository;
pub mod tts;
pub mod web_server;
pub mod extension;
pub mod script;
pub mod jni_bridge;
pub mod models;

pub fn run_cli(args: &[String]) -> anyhow::Result<()> {
    println!("legado_tauri cli ... ");
    match args.first().map(|s| s.as_str()) {
        Some("booksource-eval") => {
            println!("legado_tauri cli booksource-eval {}", args.get(1).unwrap_or(&String::new()));
        }
        Some("booksource-test") => {
            println!("legado_tauri cli booksource-test {:?}", args.get(1));
        }
        _ => {
            println!("legado_tauri cli <command>");
            println!("  booksource-eval <path>");
            println!("  booksource-test [id]");
        }
    }
    Ok(())
}

pub fn run_app() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(commands::generate_handler())
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
