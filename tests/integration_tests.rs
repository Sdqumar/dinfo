// Integration tests that test the CLI functionality

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_help_command() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--help"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("A CLI tool to analyze directory sizes"));
        assert!(stdout.contains("--files"));
        assert!(stdout.contains("--dirs"));
    }

    #[test]
    fn test_version_command() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--version"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("dinfo"));
    }

    #[test]
    fn test_invalid_directory() {
        let output = Command::new("cargo")
            .args(&["run", "--", "/nonexistent/path"])
            .output()
            .expect("Failed to execute command");

        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("does not exist"));
    }
}
