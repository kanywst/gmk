use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "gmk",
    about = "🚀 Bookmark & Interactive Git Clone Tool",
    version
)]
pub struct Cli {
    /// Subcommand
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Force overwrite existing directory
    #[arg(long, short = 'f')]
    pub force: bool,

    /// Perform a shallow clone (--depth 1)
    #[arg(long)]
    pub shallow: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Bookmark a repository
    Set {
        /// Git repository URL (https or ssh)
        url: String,
    },
    /// List all bookmarked repositories
    List,
}
