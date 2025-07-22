use super::config;
use std::path::{Path};
use std::process::Command;
use colored::*;

enum GitRepoStatus {
    Valid,
    PathNotFound,
    NotDirectory,
    NotGitRepo,
}

fn check_git_repo(path: &Path) -> GitRepoStatus {
    if !path.exists() {
        return GitRepoStatus::PathNotFound;
    }
    
    if !path.is_dir() {
        return GitRepoStatus::NotDirectory;
    }
    
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .current_dir(path)
        .output();
    
    if let Err(_) = output {
        return GitRepoStatus::NotGitRepo;
    }
    
    if !output.unwrap().status.success() {
        return GitRepoStatus::NotGitRepo;
    }
    
    GitRepoStatus::Valid
}

pub fn add_config (config: &config::AddConfig) {
    let source_path = Path::new(&config.source);
    let destination_path = Path::new(&config.destination);
    
    let configs = config::load().unwrap_or_else(|_| Vec::new());
    if configs.iter().any(|item| item.name == config.name) {
        eprint!("{}", "Name already exist: ".red());
        eprintln!("{}", config.name.red());
        return;
    }
    
    match check_git_repo(source_path) {
        GitRepoStatus::Valid => {},
        GitRepoStatus::PathNotFound => {
            eprint!("{}", "Source path not found: ".red());
            eprintln!("{}", source_path.display().to_string().red());
            return;
        },
        GitRepoStatus::NotDirectory => {
            eprint!("{}", "Source path is not a directory: ".red());
            eprintln!("{}", source_path.display().to_string().red());
            return;
        },
        GitRepoStatus::NotGitRepo => {
            eprint!("{}", "Source path is not a git repository: ".red());
            eprintln!("{}", source_path.display().to_string().red());
            return;
        },
    }
    
    match check_git_repo(destination_path) {
        GitRepoStatus::Valid => {},
        GitRepoStatus::PathNotFound => {
            eprint!("{}", "Destination path not found: ".red());
            eprintln!("{}", destination_path.display().to_string().red());
            return;
        },
        GitRepoStatus::NotDirectory => {
            eprint!("{}", "Destination path is not a directory: ".red());
            eprintln!("{}", destination_path.display().to_string().red());
            return;
        },
        GitRepoStatus::NotGitRepo => {
            eprint!("{}", "Destination path is not a git repository: ".red());
            eprintln!("{}", destination_path.display().to_string().red());
            return;
        },
    }
    
    if let Err(e) = config::add(&config) {
        eprint!("{}", "Error saving config: ".red());
        eprintln!("{}", e.to_string().red());
    }
    
    println!("{}", "Config saved".green());
}