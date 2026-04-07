use std::process::Command;

pub fn get_authors() -> Vec<String> {
    let output = Command::new("git")
        .args(["log", "--pretty=format:%an"])
        .output()
        .expect("failed to execute git");

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout.lines().map(|s| s.to_string()).collect()
}