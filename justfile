# Documentation: https://github.com/casey/just
# Cheat sheet: https://cheatography.com/linux-china/cheat-sheets/justfile/

# List all available scripts
[private]
default:
  @just --list --unsorted

# Init submodules. Warning: may discard changes
[group("configuration")]
init-submodules:
  git submodule update --init --recursive

# Format all crates in the workspace
[group("maintenance")]
format *ARGS:
  cargo +nightly fmt --all {{ARGS}}

# Lint all crates in the workspace
[group("maintenance")]
lint *ARGS:
  cargo clippy --all-targets {{ARGS}}

# Lint and fix all crates in the workspace, then format
[group("maintenance")]
fix *ARGS: (lint "--fix" ARGS) format

# Update all project dependencies (cargo and vcpkg)
# Requires cargo-edit
[group("maintenance")]
update:
  cargo upgrade -i # From cargo-edit
  cargo update

# Run the headless app
[group("development")]
run *ARGS:
  cargo run -p playstation-headless -- --bios roms/bios/scph1001.bin {{ARGS}}

# Run tests for all crates in the workspace
[group("development")]
test *ARGS:
  cargo test {{ARGS}}
