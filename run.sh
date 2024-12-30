#!/usr/bin/env bash

set -euo pipefail

# Download Alpine Linux minirootfs

alpine_x86_64_minirootfs_url="https://dl-cdn.alpinelinux.org/alpine/v3.21/releases/x86_64/alpine-minirootfs-3.21.0-x86_64.tar.gz"
output_file="alpine/alpine-minirootfs.tar.gz"
mkdir -p alpine

# If output file doesn't exist, download it
if [ ! -f "$output_file" ]; then
  echo "Downloading $alpine_x86_64_minirootfs_url to $output_file"
  curl -o "$output_file" "$alpine_x86_64_minirootfs_url"
fi

# Calculate the SHA256 checksum of the downloaded file
actual_sha=$(sha256sum "$output_file" | awk '{ print $1 }')
expected_sha="55ea3e5a7c2c35e6268c5dcbb8e45a9cd5b0e372e7b4e798499a526834f7ed90"

# Compare the checksums
if [ "$expected_sha" != "$actual_sha" ]; then
  echo "Error: Alpine minirootfs SHA256 checksum does not match."
  echo "Expected: $expected_sha"
  echo "Actual:   $actual_sha"
  exit 1
fi

# Extract the minirootfs
output_directory="alpine/alpine-minirootfs"
rm -rf "$output_directory"
mkdir -p "$output_directory"
tar -xzf "$output_file" -C alpine/alpine-minirootfs

# Build and run Rust code
cd rust/
cargo build
sudo ./target/debug/contained "$@"
