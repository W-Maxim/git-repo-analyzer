use std::collections::HashMap;
use crate::models::AuthorStats;
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