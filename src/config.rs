use clap::{Args, ValueEnum};
use serde::{Serialize, Deserialize};
use std::env;
use std::fs;
use std::path::{PathBuf};

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct AddConfig {
    /// Unique pair name
    #[arg(short, long)]
    pub name: String,
    
    /// Github source absolute folder
    #[arg(short, long)]
    pub source: String,
    
    /// Github destination absolute folder
    #[arg(short, long)]
    pub destination: String,
    
    /// Base branch
    #[arg(short, long)]
    pub branch: String,
}

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct RemoveConfig {
    /// Unique pair name
    #[arg(short, long)]
    pub name: String,
}

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Unique pair name
    pub name: String,
    
    /// Branch name
    #[arg(short, long, default_value = "main")]
    pub branch: String,
    
    /// Commit message
    #[arg(short, long)]
    pub message: String,
    
    /// Sync direction
    #[arg(short, long, default_value = "a2b")]
    pub direction: SyncDirection,
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
pub enum SyncDirection {
    A2B,
    B2A,
}

type Configs = Vec<AddConfig>;

pub fn get_path() -> Option<std::path::PathBuf> {
    // Try XDG_CONFIG_HOME first (Linux standard)
    if let Some(xdg_config) = env::var_os("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(xdg_config).join("ghsync").join("config.json"));
    }
    
    // Fallback to ~/.config
    if let Some(home) = env::var_os("HOME") {
        return Some(PathBuf::from(home).join(".config").join("ghsync").join("config.json"));
    }
    
    None
}

pub fn save(config: &Configs) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(config_path) = get_path() {
        if let Some(parent_dir) = config_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }
        
        let json = serde_json::to_string_pretty(&config)?;
        fs::write(config_path, json)?;
    }
    Ok(())
}

pub fn add(item: &AddConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut configs = load().unwrap_or_else(|_| Vec::new());
    
    configs.push(AddConfig {
        name: item.name.clone(),
        source: item.source.clone(),
        destination: item.destination.clone(),
        branch: item.branch.clone(),
    });
    
    save(&configs)
}

pub fn load() -> Result<Configs, Box<dyn std::error::Error>> {
    if let Some(config_path) = get_path() {
        let content = fs::read_to_string(config_path)?;
        let config: Configs = serde_json::from_str(&content)?;
        Ok(config)
    } else {
        Ok(Vec::new())
    }
}

pub fn remove(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load().unwrap_or_else(|_| Vec::new());
    
    if !config.iter().any(|item| item.name == name) {
        return Err(format!("Config not found: {}", name).into());
    }
    
    config.retain(|item| item.name != name);
    
    save(&config)
}