use std::collections::HashMap;
use crate::models::{AuthorStats, FileStats};
use std::cmp::Reverse;

pub fn count_authors(authors: Vec<String>) -> Vec<AuthorStats> {
    let mut author_map = HashMap::new();

    for author in authors {
        if let Some(count) = author_map.get_mut(&author) {
            *count += 1;
        } else {
            author_map.insert(author, 1);
        }
    }

    let mut stats = Vec::new();

    for (name, count) in author_map {
        stats.push(AuthorStats {
            name,
            commit_count: count,
        });
    }

    stats.sort_by_key(|a| Reverse(a.commit_count));

    stats
}

pub fn count_files(files: Vec<String>) -> Vec<FileStats> {
    let mut file_map = HashMap::new();

    for file in files {
        if let Some(count) = file_map.get_mut(&file) {
            *count += 1;
        } else {
            file_map.insert(file, 1);
        }
    }

    let mut stats = Vec::new();

    for (path, count) in file_map {
        stats.push(FileStats {
            path,
            change_count: count,
        });
    }

    stats.sort_by_key(|a| Reverse(a.change_count));

    stats
}