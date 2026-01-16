use anyhow::{Context, Result};
use colored::*;
use regex::Regex;
use std::path::Path;
use std::process::{Command, Stdio};

/// Extracts "Owner/Repo" from a Git URL.
pub fn extract_repo_name(url: &str) -> Result<String> {
    let clean_url = url.trim_end_matches(".git");
    let re = Regex::new(r"[:/](?P<owner>[^/]+)/(?P<repo>[^/]+)$")?;

    if let Some(caps) = re.captures(clean_url) {
        let owner = &caps["owner"];
        let repo = &caps["repo"];
        Ok(format!("{}/{}", owner, repo))
    } else {
        Err(anyhow::anyhow!(
            "Invalid Git URL format. Expected 'owner/repo' structure."
        ))
    }
}

/// Executes the git clone command.
pub fn run_clone(
    dir_name: &str,
    url: &str,
    branch: Option<&str>,
    force: bool,
    shallow: bool,
) -> Result<()> {
    let target_dir = dir_name.split('/').next_back().unwrap_or(dir_name);
    let target_path = Path::new(target_dir);

    if target_path.exists() {
        if force {
            println!(
                "{} Removing existing directory '{}'...",
                "🗑️".red(),
                target_dir
            );
            std::fs::remove_dir_all(target_path).context("Failed to remove existing directory")?;
        } else {
            return Err(anyhow::anyhow!(
                "Directory '{}' already exists. Use {} or {} to overwrite.",
                target_dir.bold(),
                "--force".yellow(),
                "-f".yellow()
            ));
        }
    }

    println!(
        "{} Cloning {} ({}) ...",
        "🚀".magenta(),
        dir_name.bold(),
        branch.unwrap_or("default").cyan()
    );

    let mut cmd = Command::new("git");
    cmd.arg("clone");

    if let Some(b) = branch {
        cmd.arg("-b").arg(b);
    }

    if shallow {
        cmd.arg("--depth").arg("1");
    }

    cmd.arg(url);
    cmd.arg(target_dir);

    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

    let status = cmd
        .status()
        .context("Failed to execute git command. Is git installed?")?;

    if status.success() {
        println!(
            "\n{} Successfully cloned into './{}'",
            "✨".green().bold(),
            target_dir
        );
    } else {
        return Err(anyhow::anyhow!("Git clone process failed."));
    }

    Ok(())
}
