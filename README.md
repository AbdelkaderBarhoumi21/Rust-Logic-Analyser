# Rust Log Analyzer

A simple command-line tool in Rust that reads a log file, counts the total number of lines, tallies how many times each log level appears (`ERROR`, `INFO`, `WARNING`), and reports which level is the most frequent.

## Features

- 📄 **Line-by-line file reading** with `BufReader`  
- 🔢 **Total line count**  
- 📊 **Per-level occurrence counting** using `HashMap::entry().and_modify().or_insert()`  
- 🏆 **Identification of the most common** log level  
- ❌ **Error propagation** via the `?` operator  

## Prerequisites

- Rust toolchain (1.60 or later) and Cargo  
- A log file named `output.log` in the same directory, containing lines with `ERROR`, `INFO`, or `WARNING`

## Getting Started

1. **Clone the repo**  
   ```bash
   git clone https://github.com/AbdelkaderBarhoumi21/Rust-Logic-Analyser.git
   cd rust-log-analyzer
