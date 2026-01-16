mod cli;
mod config;
mod git;
mod ui;

use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use config::AppConfig;
use directories::UserDirs;
use std::path::PathBuf;

fn get_config_path() -> Result<PathBuf> {
    let user_dirs = UserDirs::new().context("Could not determine user directories")?;
    let home_dir = user_dirs.home_dir();
    let config_dir = home_dir.join(".gmk");

    // Create directory if it doesn't exist
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)?;
    }

    Ok(config_dir.join("config.toml"))
}

fn main() -> Result<()> {
    // Note: Could add a custom panic handler here to suppress stack traces,
    // but anyhow handles errors sufficiently for this use case.

    let args = cli::Cli::parse();
    let cfg_path = get_config_path()?;
    let mut cfg: AppConfig = confy::load_path(&cfg_path).unwrap_or_default();

    match &args.command {
        Some(cli::Commands::Set { url }) => match git::extract_repo_name(url) {
            Ok(key) => {
                cfg.repos.insert(key.clone(), url.clone());
                confy::store_path(&cfg_path, &cfg)?;
                println!(
                    "{} {} {}",
                    "✔ Saved:".green().bold(),
                    key.cyan(),
                    "to bookmarks.".green()
                );
            }
            Err(e) => {
                eprintln!("{} {}", "Error:".red().bold(), e);
            }
        },

        Some(cli::Commands::List) => {
            println!("{}", "🔖 Bookmarked Repositories:".bold().underline());
            if cfg.repos.is_empty() {
                println!("  (No bookmarks yet)");
            } else {
                let mut keys: Vec<_> = cfg.repos.keys().collect();
                keys.sort();
                for key in keys {
                    let url = &cfg.repos[key];
                    println!("  {: <25} -> {}", key.cyan(), url.dimmed());
                }
            }
        }

        None => {
            if cfg.repos.is_empty() {
                println!(
                    "{}",
                    "No repositories found. Run `gmk set <url>` first.".yellow()
                );
                return Ok(());
            }

            let mut keys: Vec<String> = cfg.repos.keys().cloned().collect();
            keys.sort();

            match ui::run_fuzzy_finder(keys)? {
                ui::UserAction::Abort => {
                    // Do nothing and exit quietly
                }
                ui::UserAction::CloneDefault(key) => {
                    println!("{} {}", "Selected:".bold(), key.cyan());
                    let url = cfg.repos.get(&key).context("URL not found in config")?;
                    git::run_clone(&key, url, None, args.force, args.shallow)?;
                }
                ui::UserAction::CloneWithBranch(key) => {
                    println!("{} {}", "Selected:".bold(), key.cyan());
                    let url = cfg.repos.get(&key).context("URL not found in config")?;

                    if let Some(branch) = ui::prompt_branch_name()? {
                        git::run_clone(&key, url, Some(&branch), args.force, args.shallow)?;
                    } else {
                        println!("{}", "✖ Operation cancelled (empty branch name).".yellow());
                    }
                }
            }
        }
    }

    Ok(())
}
