# NMAP OS Scanner

A simple Rust wrapper for NMAP that processes XML output with `xq` and saves it to a file.

## Prerequisites

This tool requires the following external dependencies:
- `nmap`: Network mapper utility
- `xq`: Command-line XML and HTML processor

## Installation

Make sure you have Rust and Cargo installed, then:

```bash
cargo build --release
```

## Usage

```bash
nmap_oscanner <nmap-arguments>
```

For example:
```bash
nmap_oscanner 192.168.1.1
```

The output will be saved in the `output` directory with a filename based on the arguments provided.

## Project Structure

- `src/main.rs`: Main program source code
- `output/`: Directory where scan results are stored

## Troubleshooting

If you encounter errors, ensure that:
1. Both `nmap` and `xq` are installed and available in your PATH
2. You have permission to write to the output directory
3. You have permission to create temporary files in `/tmp/`