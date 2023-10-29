#!/bin/bash
set -euo pipefail

if [ "$#" -lt 1 ]; then
    echo "This script requires 1 argument, which is the new version to use"
    exit 1
fi

# Update version in Cargo.toml
sed -E -i "3,3 s/(version = \")(.*)(\")/\\1$1\\3/" Cargo.toml

# Update version in src/lib/lib.rs
sed -E -i "10,10 s/(\\#\\[command\\(version = \")(.*)(\"\\)\\])/\1$1\3/" src/lib/lib.rs

