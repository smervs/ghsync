mod add;
mod config;
mod sync;

use colored::*;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct App {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Add config
    Add(config::AddConfig),
    
    /// List config
    List,
    
    /// Remove config
    Remove(config::RemoveConfig),
    
    /// Synchronize
    Sync(config::SyncConfig),
}

fn main() {
    let args = App::parse();
    
    match args.cmd {
        Command::Add(args) => add::add_config(&args),
        
        Command::List => {
            if let Ok(config) = config::load() {
                for item in config {
                    println!("-----------------------");
                    println!("ID: {}", item.name.green());
                    println!("A: {}", item.source);
                    println!("B: {}", item.destination);
                    println!("Base branch: {}", item.branch);
                    println!("-----------------------");
                }
            }
        },
        
        Command::Remove(args) => {
            if let Err(e) = config::remove(&args.name) {
                println!("{}", e.to_string().red());
                return;
            }
            
            println!("{}", "Config removed".green());
        },
        
        Command::Sync(args) => {
            if let Err(e) = sync::process(&args.name, args.direction, &args.branch, &args.message) {
                println!("{}", e.to_string().red());
                return;
            }
        },
    }
}
