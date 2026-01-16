use anyhow::Result;
use colored::*;
use skim::prelude::*;
use std::io::{self, Cursor, Write};

pub enum UserAction {
    CloneDefault(String),
    CloneWithBranch(String),
    Abort,
}

/// Runs the interactive fuzzy finder.
pub fn run_fuzzy_finder(keys: Vec<String>) -> Result<UserAction> {
    let input = keys.join("\n");
    // Fix: Set fixed height (10 lines)
    let height = 10;
    let height_str = format!("{}", height);

    let options = SkimOptionsBuilder::default()
        .height(Some(&height_str))
        .reverse(true)
        .multi(false)
        .prompt(Some("Clone ❯ "))
        .header(Some(
            "Select a repository (Enter: Default, Ctrl-b: Specify Branch)",
        ))
        .bind(vec!["ctrl-b:accept"])
        .build()
        .map_err(|e| anyhow::anyhow!("Skim failed: {}", e))?;

    // Fix: Save cursor position before execution (\x1B7)
    let mut stdout = io::stdout();
    write!(stdout, "\x1B7")?;
    stdout.flush()?;

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    let output = Skim::run_with(&options, Some(items))
        .ok_or_else(|| anyhow::anyhow!("Skim failed to run"))?;

    // Fix: Restore cursor position (\x1B8) and clear screen down (\x1B[J)
    // This ensures the Skim interface is completely removed and the Clone log starts from that position.
    write!(stdout, "\x1B8\x1B[J")?;
    stdout.flush()?;

    if output.is_abort {
        return Ok(UserAction::Abort);
    }

    let selected_items = output.selected_items;
    if selected_items.is_empty() {
        return Ok(UserAction::Abort);
    }

    let selected_key = selected_items[0].output().to_string();

    match output.final_key {
        Key::Ctrl('b') => Ok(UserAction::CloneWithBranch(selected_key)),
        _ => Ok(UserAction::CloneDefault(selected_key)),
    }
}

/// Prompts the user for a branch name.
pub fn prompt_branch_name() -> Result<Option<String>> {
    print!("{}", "🌿 Enter branch name: ".bold().blue());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();

    if trimmed.is_empty() {
        Ok(None)
    } else {
        Ok(Some(trimmed.to_string()))
    }
}
