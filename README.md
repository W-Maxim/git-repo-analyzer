# git-repo-analyzer

This is a small CLI tool written in Rust that analyzes a Git repository.

## Right now it can show:

how many commits each author has
which files are changed most often (hotspots)

## Requirements
Git must be installed
You need to run the tool inside a Git repository

## Requirements
Git must be installed
You need to run the tool inside a Git repository

---
## Commands
### authors

Shows how many commits each author has.

Example:

cargo run --bin git_repo_analyzer -- authors

Output looks like:

1. Maxim - 12 commits
2. Thomas - 7 commits

### hotspots

Shows which files are changed most often in the repository.

Example:

cargo run --bin git_repo_analyzer -- hotspots

Output looks like:

1. src/main.rs - 18 changes
2. src/lib.rs - 12 changes