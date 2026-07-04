//! 数据模型 — 从 .so 提取的全部结构体
//!
//! 共 58 个结构体:
//! - AddBookPayload (8 fields)
//! - AudioCacheRequest (3 fields)
//! - BaseOptions (1 fields)
//! - BaseReq (2 fields)
//! - BookInfoReq (4 fields)
//! - BrowserBridgeMessage (7 fields)
//! - BrowserBridgeResponse (4 fields)
//! - BrowserCookie (8 fields)
//! - BrowserCookieStore (2 fields)
//! - BrowserDomReadyMessage (2 fields)
//! - BrowserEvalMessage (5 fields)
//! - CachedChapter (4 fields)
//! - ChapterContentReq (5 fields)
//! - ChapterListReq (4 fields)
//! - CookieEntry (2 fields)
//! - CopyFileOptions (2 fields)
//! - CoverCacheRequest (3 fields)
//! - DeepLinkProtocol (4 fields)
//! - DeviceCodeResponse (5 fields)
//! - DeviceInfo (3 fields)
//! - DialogFilter (2 fields)
//! - EpisodeProgress (3 fields)
//! - ExploreReq (6 fields)
//! - FrontendPluginHttpRequest (5 fields)
//! - GetCurrentResponse (1 fields)
//! - GetFileNameFromUriResponse (1 fields)
//! - LogicalPosition (2 fields)
//! - LogicalSize (2 fields)
//! - Message (6 fields)
//! - OpenDialogOptions (9 fields)
//! - PathResponse (1 fields)
//! - PcsFileInfo (6 fields)
//! - PhysicalPosition (2 fields)
//! - PhysicalSize (2 fields)
//! - PrefetchChaptersPayload (10 fields)
//! - PreventOverflowMargin (2 fields)
//! - ReaderSession (9 fields)
//! - RepoManifest (4 fields)
//! - RepoSourceInfo (14 fields)
//! - RequestEventMessage (10 fields)
//! - RunTestsRequest (4 fields)
//! - SaveDialogOptions (4 fields)
//! - SaveFileResponse (1 fields)
//! - SearchReq (5 fields)
//! - ShelfBook (22 fields)
//! - ShowMessageDialogResponse (1 fields)
//! - SourceSwitchBackup (2 fields)
//! - SyncConflict (8 fields)
//! - SyncV2BookSettings (4 fields)
//! - SyncV2ReadingProgress (9 fields)
//! - SyncV2ShelfBook (18 fields)
//! - TokenResponse (4 fields)
//! - TxtChapterEntry (2 fields)
//! - UpdateShelfBookPayload (23 fields)
//! - UserFontMeta (5 fields)
//! - WindowConfig (61 fields)
//! - WindowEffectsConfig (4 fields)
//! - WindowSizeConstraints (4 fields)

use serde::{Deserialize, Serialize};

/// AddBookPayload (8 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddBookPayload {
    // 从二进制提取到 8 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// AudioCacheRequest (3 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AudioCacheRequest {
    // 从二进制提取到 3 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// BaseOptions (1 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseOptions {
    // 从二进制提取到 1 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// BaseReq (2 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseReq {
    // 从二进制提取到 2 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// BookInfoReq (4 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BookInfoReq {
    // 从二进制提取到 4 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// BrowserBridgeMessage (7 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BrowserBridgeMessage {
    // 从二进制提取到 7 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// BrowserBridgeResponse (4 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BrowserBridgeResponse {
    // 从二进制提取到 4 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// BrowserCookie (8 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BrowserCookie {
    // 从二进制提取到 8 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// BrowserCookieStore (2 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BrowserCookieStore {
    // 从二进制提取到 2 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// BrowserDomReadyMessage (2 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BrowserDomReadyMessage {
    // 从二进制提取到 2 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// BrowserEvalMessage (5 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BrowserEvalMessage {
    // 从二进制提取到 5 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// CachedChapter (4 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CachedChapter {
    // 从二进制提取到 4 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// ChapterContentReq (5 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChapterContentReq {
    // 从二进制提取到 5 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// ChapterListReq (4 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChapterListReq {
    // 从二进制提取到 4 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// CookieEntry (2 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CookieEntry {
    // 从二进制提取到 2 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// CopyFileOptions (2 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CopyFileOptions {
    // 从二进制提取到 2 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// CoverCacheRequest (3 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoverCacheRequest {
    // 从二进制提取到 3 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// DeepLinkProtocol (4 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeepLinkProtocol {
    // 从二进制提取到 4 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// DeviceCodeResponse (5 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeviceCodeResponse {
    // 从二进制提取到 5 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// DeviceInfo (3 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeviceInfo {
    // 从二进制提取到 3 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// DialogFilter (2 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DialogFilter {
    // 从二进制提取到 2 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// EpisodeProgress (3 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EpisodeProgress {
    // 从二进制提取到 3 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// ExploreReq (6 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExploreReq {
    // 从二进制提取到 6 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// FrontendPluginHttpRequest (5 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FrontendPluginHttpRequest {
    // 从二进制提取到 5 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// GetCurrentResponse (1 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetCurrentResponse {
    // 从二进制提取到 1 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// GetFileNameFromUriResponse (1 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetFileNameFromUriResponse {
    // 从二进制提取到 1 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// LogicalPosition (2 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LogicalPosition {
    // 从二进制提取到 2 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// LogicalSize (2 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LogicalSize {
    // 从二进制提取到 2 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// Message (6 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    // 从二进制提取到 6 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// OpenDialogOptions (9 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OpenDialogOptions {
    // 从二进制提取到 9 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// PathResponse (1 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PathResponse {
    // 从二进制提取到 1 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// PcsFileInfo (6 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PcsFileInfo {
    // 从二进制提取到 6 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// PhysicalPosition (2 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhysicalPosition {
    // 从二进制提取到 2 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// PhysicalSize (2 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhysicalSize {
    // 从二进制提取到 2 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// PrefetchChaptersPayload (10 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PrefetchChaptersPayload {
    // 从二进制提取到 10 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// PreventOverflowMargin (2 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PreventOverflowMargin {
    // 从二进制提取到 2 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// ReaderSession (9 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReaderSession {
    // 从二进制提取到 9 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// RepoManifest (4 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RepoManifest {
    // 从二进制提取到 4 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// RepoSourceInfo (14 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RepoSourceInfo {
    // 从二进制提取到 14 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// RequestEventMessage (10 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RequestEventMessage {
    // 从二进制提取到 10 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// RunTestsRequest (4 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RunTestsRequest {
    // 从二进制提取到 4 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// SaveDialogOptions (4 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SaveDialogOptions {
    // 从二进制提取到 4 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// SaveFileResponse (1 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SaveFileResponse {
    // 从二进制提取到 1 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// SearchReq (5 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchReq {
    // 从二进制提取到 5 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// ShelfBook (22 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShelfBook {
    // 从二进制提取到 22 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// ShowMessageDialogResponse (1 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShowMessageDialogResponse {
    // 从二进制提取到 1 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// SourceSwitchBackup (2 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SourceSwitchBackup {
    // 从二进制提取到 2 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// SyncConflict (8 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SyncConflict {
    // 从二进制提取到 8 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// SyncV2BookSettings (4 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SyncV2BookSettings {
    // 从二进制提取到 4 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// SyncV2ReadingProgress (9 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SyncV2ReadingProgress {
    // 从二进制提取到 9 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// SyncV2ShelfBook (18 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SyncV2ShelfBook {
    // 从二进制提取到 18 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// TokenResponse (4 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenResponse {
    // 从二进制提取到 4 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// TxtChapterEntry (2 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TxtChapterEntry {
    // 从二进制提取到 2 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// UpdateShelfBookPayload (23 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateShelfBookPayload {
    // 从二进制提取到 23 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// UserFontMeta (5 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserFontMeta {
    // 从二进制提取到 5 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// WindowConfig (61 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindowConfig {
    // 从二进制提取到 61 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// WindowEffectsConfig (4 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindowEffectsConfig {
    // 从二进制提取到 4 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
/// WindowSizeConstraints (4 个字段)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindowSizeConstraints {
    // 从二进制提取到 4 个字段，具体字段名见下方注释
    // 字段名候选 (camelCase): 从 .so 提取
}
