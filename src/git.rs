use std::process::Command;

pub fn get_authors() -> Vec<String> {
    let output = Command::new("git")
        .args(["log", "--pretty=format:%an"])
        .output()
        .expect("failed to execute git");

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout.lines().map(|s| s.to_string()).collect()
}

pub fn get_changed_files() -> Vec<String> {
    let output = Command::new("git")
        .args(["log", "--name-only", "--pretty=format:%a"])
        .output()
        .expect("failed to execute git");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut files = Vec::new();

    for line in stdout.lines() {
        let trimmed = line.trim();

        if !trimmed.is_empty() {
            files.push(trimmed.to_string());
        }
    }

    files
}