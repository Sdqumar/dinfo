mod analyzer;
mod cli;
mod formatter;

use anyhow::{Context, Result};

use analyzer::analyze_directory;
use cli::Args;
use formatter::display_results;

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Validate the path
    args.validate_path()?;

    let analysis = analyze_directory(&args.path, args.show_hidden)
        .with_context(|| format!("Failed to analyze directory: {}", args.path.display()))?;

    display_results(&analysis, args.top_files, args.top_dirs, args.show_all, args.no_color)?;

    Ok(())
}

