# dinfo

A simple Rust command-line tool to calculate and display directory sizes in human-readable format.

## What it does

`dinfo` calculates the total size of all files in the current directory and displays it in a human-readable format (B, KB, MB, GB, TB).

## Installation

### Install from source
```bash
# Clone or navigate to the project directory
cd dinfo

# Install using cargo
cargo install --path .
```

## Usage

Simply run the command in any directory to get its total size:

```bash
dinfo
```

**Output example:**
```
Dir size is 2.45 MB
```
