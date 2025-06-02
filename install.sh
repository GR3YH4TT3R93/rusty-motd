#!/bin/bash

# Set error handling for robustness
set -euo pipefail

# Define paths using variables for better maintainability
MOTD_DIR="$PREFIX/etc/motd"
MOTD_INIT="$MOTD_DIR/init"
ZSH_PROFILE="$PREFIX/etc/zprofile"
BASH_PROFILE="$PREFIX/etc/profile"

# Check if motd directory or file already exists
if [[ -e "$MOTD_DIR" ]]; then
    echo "Warning: MOTD directory/file already exists at $MOTD_DIR"
    printf "Remove existing installation and continue? (y/N): " > /dev/tty
    read < /dev/tty
    if [[ $REPLY == [Yy]* ]]; then
        echo "Removing existing MOTD file/folder..."
        rm -rf "$MOTD_DIR"
    else
        echo "Installation cancelled."
        exit 0
    fi
fi

# Clone repository (with error handling)
echo "Installing rusty-motd..."
if ! git clone https://github.com/GR3YH4TT3R93/rusty-motd.git "$MOTD_DIR"; then
    echo "Error: Failed to clone repository" >&2
    exit 1
fi

# Build Rust Components
if [[ -d "$MOTD_DIR/src" && -f "$MOTD_DIR/Cargo.toml" ]]; then
    echo "Building Rust components..."
    (
        cd "$MOTD_DIR"
        cargo build --release
        # Move binaries to maintain your existing structure
        cp ./target/release/rusty-motd $MOTD_INIT
        rm -rf target   # Clean up build directory
    )
fi

# Determine profile file using parameter expansion instead of basename subprocess
case "${SHELL##*/}" in
    zsh)
        profile_file="$ZSH_PROFILE"
        ;;
    *)
        profile_file="$BASH_PROFILE"
        ;;
esac

# Check if init script already exists in profile using built-in grep
if ! grep -Fq "$MOTD_INIT" "$profile_file" 2>/dev/null; then
    echo "Adding MOTD initialization to $profile_file"
    echo "$MOTD_INIT" >> "$profile_file"
    echo "MOTD installed successfully!"
else
    echo "MOTD initialization already present in $profile_file"
fi
