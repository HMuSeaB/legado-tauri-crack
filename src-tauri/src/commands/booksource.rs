//! booksource 命令模块
//!
//! 实现真实的 Legado 开源阅读书源导入、本地持久化及读取

use serde_json::{json, Value};
use std::fs;
use tauri::Manager;

/// 获取书源列表
pub async fn real_booksource_list(app_handle: &tauri::AppHandle) -> anyhow::Result<Value> {
    let app_data_dir = app_handle.path().app_data_dir()?;
    let booksources_dir = app_data_dir.join("booksources");
    
    if !booksources_dir.exists() {
        return Ok(json!([]));
    }
    
    let mut list = Vec::new();
    if let Ok(entries) = fs::read_dir(booksources_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(val) = serde_json::from_str::<Value>(&content) {
                            list.push(val);
                        }
                    }
                }
            }
        }
    }
    
    Ok(Value::Array(list))
}

/// 从远程 URL 导入书源
pub async fn real_import_legacy_json_url(
    app_handle: &tauri::AppHandle,
    url: &str,
    _smart_explore: bool,
) -> anyhow::Result<Value> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;
        
    let res = client.get(url).send().await?.text().await?;
    real_import_legacy_json_text(app_handle, &res, _smart_explore).await
}

/// 解析书源文本内容并写入本地
pub async fn real_import_legacy_json_text(
    app_handle: &tauri::AppHandle,
    content: &str,
    _smart_explore: bool,
) -> anyhow::Result<Value> {
    let app_data_dir = app_handle.path().app_data_dir()?;
    let booksources_dir = app_data_dir.join("booksources");
    
    if !booksources_dir.exists() {
        fs::create_dir_all(&booksources_dir)?;
    }
    
    let json_val: Value = serde_json::from_str(content)?;
    let mut imported = 0;
    let mut files = Vec::new();
    
    let items = if let Some(arr) = json_val.as_array() {
        arr.clone()
    } else {
        vec![json_val]
    };
    
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;
        
    for item in items {
        let name = item.get("bookSourceName")
            .and_then(|v| v.as_str())
            .unwrap_or("未命名书源");
            
        let url = item.get("bookSourceUrl")
            .and_then(|v| v.as_str())
            .unwrap_or("");
            
        let uuid = item.get("uuid")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
            
        let source_type = item.get("bookSourceType")
            .and_then(|v| {
                if let Some(n) = v.as_i64() {
                    Some(match n {
                        1 => "comic",
                        2 => "video",
                        3 => "music",
                        _ => "novel",
                    })
                } else {
                    v.as_str()
                }
            })
            .unwrap_or("novel");
            
        // 构造前端需要的 BookSourceMeta 元数据结构
        let meta = json!({
            "sourceKey": uuid,
            "uuid": uuid,
            "fileName": format!("{}.json", uuid),
            "name": name,
            "url": url,
            "urls": [url],
            "enabled": true,
            "fileSize": 0,
            "modifiedAt": now_ms,
            "sourceDir": booksources_dir.to_string_lossy().to_string(),
            "sourceType": source_type,
            "version": "1.0.0",
            "tags": []
        });
        
        let file_path = booksources_dir.join(format!("{}.json", uuid));
        fs::write(&file_path, serde_json::to_string_pretty(&meta)?)?;
        
        imported += 1;
        files.push(format!("{}.json", uuid));
    }
    
    Ok(json!({
        "imported": imported,
        "skipped": 0,
        "files": files,
        "errors": Vec::<String>::new()
    }))
}

// 占位空函数，保持与骨架结构编译兼容
pub async fn booksource_dirs() -> anyhow::Result<()> { Ok(()) }
pub async fn booksource_eval() -> anyhow::Result<()> { Ok(()) }
pub async fn booksource_pick_dir() -> anyhow::Result<()> { Ok(()) }
pub async fn booksource_remove_dirbooksource_pick_dir() -> anyhow::Result<()> { Ok(()) }
pub async fn booksource_test() -> anyhow::Result<()> { Ok(()) }
