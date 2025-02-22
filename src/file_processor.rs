use crate::filters::should_exclude;
use crate::language_detection::detect_language;
use crate::text_processing::condense;
use rayon::prelude::*;
use std::fs;
use walkdir::WalkDir;

#[derive(Debug, serde::Serialize)]
pub struct FileData {
    pub path: String,
    pub language: String,
    pub content: String,
}

pub fn process_directory(directory: &str) -> Vec<FileData> {
    let entries: Vec<_> = WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| !should_exclude(e))
        .collect();

    entries
        .par_iter()
        .filter_map(|entry| {
            fs::read_to_string(entry.path())
                .ok()
                .map(|s| FileData {
                    path: entry.path().to_string_lossy().to_string(),
                    language: detect_language(entry.path()),
                    content: condense(&s),
                })
        })
        .collect()
}
