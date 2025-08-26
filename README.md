# dinfo 📁

A fast and feature-rich Rust command-line tool for analyzing directory sizes and finding the largest files and folders.

![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)

## ✨ Features

- 🚀 **Fast directory analysis** with parallel processing
- 📊 **Detailed statistics** showing total size, file count, and directory count
- 🎯 **Top files and folders** identification
- 🌈 **Colorized output** for better readability (can be disabled)
- 🔍 **Hidden files support** with `--hidden` flag
- 📏 **Customizable limits** for number of results to show
- 📂 **Any directory** analysis (not just current directory)
- 🛡️ **Robust error handling** with meaningful error messages
- 💻 **Cross-platform** support (Linux, macOS, Windows)

## 🚀 Installation

### Install from source
```bash
# Clone or navigate to the project directory
cd dinfo

# Install using cargo
cargo install --path .
```

### Install from crates.io (when published)
```bash
cargo install dinfo
```

## 📖 Usage

### Basic usage
Analyze the current directory:
```bash
dinfo
```

### Analyze a specific directory
```bash
dinfo /path/to/directory
dinfo ~/Downloads
dinfo ./src
```

### Command-line options

```bash
dinfo [OPTIONS] [PATH]
```

#### Arguments:
- `[PATH]` - Directory to analyze (defaults to current directory)

#### Options:
- `-f, --files <NUMBER>` - Number of top files to show (default: 10)
- `-d, --dirs <NUMBER>` - Number of top directories to show (default: 10)
- `-a, --all` - Show all files and directories (no limit)
- `--no-color` - Disable colored output
- `--hidden` - Include hidden files and directories
- `-h, --help` - Print help information
- `-V, --version` - Print version information

### Examples

#### Show top 20 files and 15 directories
```bash
dinfo -f 20 -d 15
```

#### Analyze with hidden files included
```bash
dinfo --hidden
```

#### Show all results without color
```bash
dinfo --all --no-color
```

#### Analyze a specific directory with custom limits
```bash
dinfo ~/Documents -f 5 -d 3
```

## 📋 Sample Output

```
📁 Directory Analysis Summary
==================================================
 Total Size: 2.45 GB
      Files: 1,247
Directories: 89

📄 Top 10 Largest Files
------------------------------
  1.    524.3 MB ./target/release/deps/libsome_crate.rlib
  2.    312.7 MB ./dataset/training_data.csv
  3.    156.2 MB ./docs/manual.pdf
  4.     89.4 MB ./src/assets/video.mp4
  5.     45.1 MB ./logs/application.log

📂 Top 10 Largest Directories
------------------------------
  1.      1.2 GB ./target
  2.    678.9 MB ./node_modules
  3.    234.5 MB ./dataset
  4.    123.4 MB ./docs
  5.     67.8 MB ./src
```

## 🔧 Development

### Prerequisites
- Rust 1.70 or higher
- Cargo

### Building from source
```bash
git clone https://github.com/yourusername/dinfo
cd dinfo
cargo build --release
```

### Running tests
```bash
cargo test
```

### Running with development features
```bash
cargo run -- --help
cargo run -- . -f 5 --hidden
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [clap](https://github.com/clap-rs/clap) for command-line parsing
- Uses [walkdir](https://github.com/BurntSushi/walkdir) for directory traversal
- Uses [colored](https://github.com/colored-rs/colored) for terminal colors
