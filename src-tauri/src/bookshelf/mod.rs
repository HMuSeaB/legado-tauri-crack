//! 书架模块
//!
//! 命令 (6 个):
//! - bookshelf_change
//! - bookshelf_export_booksync_baidu_poll_tokendevice
//! - bookshelf_get_episode_progresswss
//! - bookshelf_meta
//! - bookshelf_pick_save_pathdefault
//! - bookshelf_removeread

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShelfBook {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub cover: Option<String>,
    pub intro: Option<String>,
    pub book_url: String,
    pub source_name: String,
    pub last_chapter: Option<String>,
    pub added_at: i64,
    pub last_read_at: Option<i64>,
    pub read_chapter_url: Option<String>,
    pub total_chapters: u32,
    pub source_type: String,
    pub read_page_index: u32,
    pub read_scroll_ratio: f64,
    pub reader_settings: Option<String>,
    pub is_private: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedChapter {
    pub chapter_index: u32,
    pub chapter_url: String,
    pub content: String,
    pub cached_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeProgress {
    pub book_id: i64,
    pub chapter_index: u32,
    pub playback_time: f64,
    pub last_played_at: i64,
}
