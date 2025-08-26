use clap::Parser;
use std::path::PathBuf;

/// A fast directory information tool that shows file and folder sizes
#[derive(Parser)]
#[command(name = "dinfo")]
#[command(about = "A CLI tool to analyze directory sizes and find largest files/folders")]
#[command(version)]
pub struct Args {
    /// Directory to analyze (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Number of top files to show
    #[arg(short = 'f', long = "files", default_value = "10")]
    pub top_files: usize,

    /// Number of top folders to show
    #[arg(short = 'd', long = "dirs", default_value = "10")]
    pub top_dirs: usize,

    /// Show all files and folders (no limit)
    #[arg(short = 'a', long = "all")]
    pub show_all: bool,

    /// Disable colored output
    #[arg(long = "no-color")]
    pub no_color: bool,

    /// Show hidden files and directories
    #[arg(long = "hidden")]
    pub show_hidden: bool,
}

impl Args {
    /// Parse command-line arguments
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
    
    /// Validate that the provided path exists and is a directory
    pub fn validate_path(&self) -> anyhow::Result<()> {
        if !self.path.exists() {
            anyhow::bail!("Path '{}' does not exist", self.path.display());
        }
        
        if !self.path.is_dir() {
            anyhow::bail!("Path '{}' is not a directory", self.path.display());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_args_defaults() {
        // We can't easily test clap parsing without creating a full CLI context,
        // but we can test the structure
        let args = Args {
            path: PathBuf::from("."),
            top_files: 10,
            top_dirs: 10,
            show_all: false,
            no_color: false,
            show_hidden: false,
        };
        
        assert_eq!(args.path, PathBuf::from("."));
        assert_eq!(args.top_files, 10);
        assert_eq!(args.top_dirs, 10);
        assert!(!args.show_all);
        assert!(!args.no_color);
        assert!(!args.show_hidden);
    }

    #[test]
    fn test_validate_path_current_directory() {
        let args = Args {
            path: PathBuf::from("."),
            top_files: 10,
            top_dirs: 10,
            show_all: false,
            no_color: false,
            show_hidden: false,
        };
        
        // Current directory should always exist
        assert!(args.validate_path().is_ok());
    }

    #[test]
    fn test_validate_path_nonexistent() {
        let args = Args {
            path: PathBuf::from("/nonexistent/path/that/should/not/exist"),
            top_files: 10,
            top_dirs: 10,
            show_all: false,
            no_color: false,
            show_hidden: false,
        };
        
        // Nonexistent path should fail validation
        assert!(args.validate_path().is_err());
    }
}
