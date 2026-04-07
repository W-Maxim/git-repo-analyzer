use std::collections::HashMap;

pub fn count_authors(authors: Vec<String>) -> HashMap<String, usize> {
    let mut authorMap = HashMap::new();

    for author in authors {
        *authorMap.entry(author).or_insert(0) += 1;
    }

    authorMap
}