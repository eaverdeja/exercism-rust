#!/bin/bash

# Find all directories that contain Cargo.toml files (which indicates they are Rust projects)
echo "# Auto-generated with update-workspace.sh! Do not edit manually" > Cargo.toml
echo "[workspace]" >> Cargo.toml
echo "members = [" >> Cargo.toml

# Find all subdirectories with Cargo.toml files
find . -mindepth 2 -maxdepth 2 -name "Cargo.toml" -type f | \
    sed -e 's|^./||' -e 's|/Cargo.toml$||' | \
    sort | \
    while read dir; do
        # Skip directories starting with dots and the target directory
        if [[ "$dir" != .* && "$dir" != "target" ]]; then
            echo "    \"$dir\"," >> Cargo.toml
        fi
    done

echo "]" >> Cargo.toml

echo "Workspace configuration updated in Cargo.toml"