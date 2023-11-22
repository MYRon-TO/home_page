pub mod walker_blogs;
pub mod walker_series;

use std::sync::Arc;
use walkdir::{DirEntry, WalkDir};

// use serde::{Deserialize, Serialize};
use crate::Database;

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
