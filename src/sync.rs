use crate::config::SyncDirection;

use super::config;
use std::io::{self, Write};
use std::process::Command;
use std::path::{Path};
use colored::*;

fn ask_yes_or_no(prompt: &str) -> bool {
    loop {
        print!("{} [y/n]: ", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            _ => return false,
        }
    }
}

fn copy(source: &str, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source_with_slash = if source.ends_with('/') {
        source.to_string()
    } else {
        format!("{}/", source)
    };

    let output = Command::new("rsync")
        .arg("-av")
        .arg("--delete")
        .arg("--exclude")
        .arg(".*") // hidden files
        .arg("--exclude")
        .arg(".*.") // hidden directories
        .arg(source_with_slash)
        .arg(destination)
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Error: {}", error).into());
    }
    
    Ok(())
}

fn fetch(source: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(source);

    let output = Command::new("git")
        .arg("fetch")
        .current_dir(path)
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Error: {}", error).into());
    }
    
    Ok(())
}

fn checkout_and_pull(source: &str, branch: &str, create_branch: bool) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(source);
    
    let output = Command::new("git")
        .arg("checkout")
        .arg(branch)
        .current_dir(path)
        .output()?;
    
    if !output.status.success() {
        if !create_branch {
            return Err(format!("Branch not found: {}", branch).into());
        }

        let output = Command::new("git")
            .arg("checkout")
            .arg("-b")
            .arg(branch)
            .current_dir(path)
            .output();
        
        if let Err(_) = output {
            return Err(format!("Error creating branch: {}", branch).into());
        }
        
        let push_output = Command::new("git")
            .arg("push")
            .arg("-u")
            .arg("origin")
            .arg(branch)
            .current_dir(path)
            .output()?;
        
        if !push_output.status.success() {
            let error = String::from_utf8_lossy(&push_output.stderr);
            return Err(format!("Failed to push new branch '{}': {}", branch, error.trim()).into());
        }
    }
    
    let output = Command::new("git")
        .arg("pull")
        .current_dir(path)
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Error: {}", error).into());
        
    }

    Ok(())
}

fn commit_and_push(destination: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(destination);
    
    let output = Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(path)
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Error: {}", error).into());
    }
    
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .current_dir(path)
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Error: {}", error).into());
    }
    
    let output = Command::new("git")
        .arg("push")
        .current_dir(path)
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Error: {}", error).into());
    }
    
    Ok(())
}

fn get_org_and_repo(source: &str) -> Option<String> {
    let path = Path::new(source);
    let output = Command::new("git")
        .args(["config", "--get", "remote.origin.url"])
        .current_dir(path)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let url = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let cleaned = url
        .replace("https://", "")
        .replace("git@", "")
        .replace("github.com:", "")
        .replace("github.com/", "");

    let parts: Vec<&str> = cleaned
        .trim_end_matches(".git")
        .split('/')
        .collect();

    if parts.len() >= 2 {
        Some(format!("{}/{}", parts[0], parts[1]))
    } else {
        None
    }
}

fn has_git_changes(source: &str) -> bool {
    let path = Path::new(source);
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(path)
        .output();
    
    if let Err(_) = output {
        return false;
    }
    
    !output.unwrap().stdout.is_empty()
}

pub fn process(name: &str, sycn_direction: SyncDirection, branch:  &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let configs = config::load().unwrap_or_else(|_| Vec::new());
    
    let sync_config = configs.
        iter()
        .find(|item| item.name == name)
        .ok_or_else(|| format!("Config not found: {}", name))?;
    
    let direction = match sycn_direction {
        SyncDirection::A2B => "a2b",
        SyncDirection::B2A => "b2a",
    };
    
    let source = if direction == "a2b" { &sync_config.source } else { &sync_config.destination };
    let destination = if direction == "a2b" { &sync_config.destination } else { &sync_config.source };
    
    println!("Syncing");
    println!("A: {}", sync_config.source);
    println!("B: {}", sync_config.destination);
    println!("Branch: {}", branch.green());
    println!("Direction: {}\n", if direction == "a2b" { "A -> B" } else { "B -> A" });
    
    if !ask_yes_or_no("Are you sure?") {
        return Err(format!("Aborted").into());
    }
    
    let source_name = get_org_and_repo(&source);
    if source_name.is_none() {
        return Err(format!("Could not get org and repo").into());
    }

    let dest_name = get_org_and_repo(&destination);
    if source_name.is_none() {
        return Err(format!("Could not get org and repo").into());
    }
    
    println!("Fetching source {} ...", source_name.unwrap());
    fetch(&source)?;
    
    println!("Fetching destination {} ...\n", dest_name.unwrap());
    fetch(&destination)?;
    
    println!("Checking out source base({}) branch ...", sync_config.branch);
    checkout_and_pull(&source, &sync_config.branch, false)?;
    println!("Checking out destination base({}) branch ...\n", sync_config.branch);
    checkout_and_pull(&destination, &sync_config.branch, false)?;
    
    println!("Checking out source {} branch ...", branch);
    checkout_and_pull(&source, &branch, false)?;
    println!("Checking out or creating destination {} branch ...\n", branch);
    checkout_and_pull(&destination, &branch, true)?;

    println!("Syncing files...");
    copy(&source, &destination)?;
    
    if !has_git_changes(&destination) {
        println!("{}", "No changes to push".red());
        return Ok(());
    }
    
    println!("Pushing...");
    commit_and_push(&destination, &message)?;
    
    println!("{}", "Done!".green());
    Ok(())
}