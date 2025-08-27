# dinfo

[![Crates.io](https://img.shields.io/crates/v/dinfo.svg)](https://crates.io/crates/dinfo)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021+-orange.svg)](https://www.rust-lang.org)

A fast and colorful Rust command-line tool for disk usage analysis. **dinfo** provides comprehensive directory analysis with beautiful colored output, showing total size, top files, and top folders.

## ✨ Features

- 📊 **Total directory size calculation** with human-readable format
- 📁 **Top 5 largest folders** analysis
- 📄 **Top 5 largest files** identification
- 🎨 **Colorized output** for better readability
- ⚡ **Fast recursive scanning** using efficient algorithms
- 🔧 **Zero configuration** - works out of the box

## 📦 Installation

### From crates.io (Recommended)
```bash
cargo install dinfo
```

### From source
```bash
# Clone the repository
git clone https://github.com/Sdqumar/dinfo.git
cd dinfo

# Install locally
cargo install --path .
```

### Build from source
```bash
# Clone and build
git clone https://github.com/Sdqumar/dinfo.git
cd dinfo
cargo build --release

# The binary will be available at target/release/dinfo
```

## 🚀 Usage

Simply run `dinfo` in any directory to analyze its disk usage:

```bash
dinfo
```

## 📋 Output Example

```
=== Summary ===
Total size : 245.67 MB

== Top 5 Files ==
   1.     25.4 MB  ./target/release/dinfo
   2.     12.8 MB  ./assets/large_dataset.json
   3.      8.9 MB  ./docs/manual.pdf
   4.      5.2 MB  ./src/data/sample.csv
   5.      3.1 MB  ./README.md

== Top 5 Folders ==
   1.    156.2 MB  ./target
   2.     45.7 MB  ./assets
   3.     23.1 MB  ./docs
   4.     12.4 MB  ./src
   5.      8.3 MB  ./.git
```

## 🎯 Use Cases

- **Clean up disk space**: Quickly identify the largest files and folders consuming disk space
- **Project analysis**: Understand the size distribution of your codebase or project
- **System monitoring**: Regular disk usage monitoring and cleanup
- **Development workflow**: Identify bloated build artifacts or large dependencies

## 🛠️ Technical Details

- **Language**: Rust 🦀
- **Dependencies**: 
  - `walkdir` for efficient directory traversal
  - `colored` for terminal color output
- **Performance**: Optimized with heap-based algorithms for top-N selection
- **Compatibility**: Cross-platform (Linux, macOS, Windows)

## 🔧 Configuration

**dinfo** works with zero configuration. It analyzes the current working directory and displays results immediately.

## 📖 Algorithm

1. **Recursive Scanning**: Uses `walkdir` to efficiently traverse the directory tree
2. **Size Calculation**: Accumulates file sizes and propagates to parent directories
3. **Top-N Selection**: Uses binary heaps for efficient identification of largest items
4. **Human-Readable Formatting**: Converts bytes to appropriate units (B, KB, MB, GB, TB, PB)

## 🤝 Contributing

Contributions are welcome! Please feel free to:

- 🐛 Report bugs
- 💡 Suggest new features
- 🔧 Submit pull requests
- 📖 Improve documentation

## 📜 License

This project is dual-licensed under:

- MIT License
- Apache License 2.0

Choose the license that best fits your needs.

## 🔗 Links

- [Crates.io page](https://crates.io/crates/dinfo)
- [GitHub repository](https://github.com/Sdqumar/dinfo)

---

**Made with ❤️ in Rust**
