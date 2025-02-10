# PatchNix

PatchNix is a tool that patches Linux binaries to work on NixOS. It analyzes the binary to find library dependencies and attempts to patch the binary by replacing paths with the provided NixOS library paths or common system library paths.

## Features

- Automatically detects missing libraries in the binary.
- Searches for the missing libraries in user-provided paths as well as common system library paths.
- Patches the binary without manual intervention.
- Provides meaningful error messages and logs the patching process.

## Requirements

- Rust and Cargo installed on your system.
- `ldd` command available on your system.

## Installation

Clone the repository and build the project using Cargo:

```sh
git clone https://github.com/yourusername/patchnix.git
cd patchnix
cargo build --release
