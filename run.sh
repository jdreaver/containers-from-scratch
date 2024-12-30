#!/usr/bin/env bash

set -euo pipefail

cd rust/
cargo build
sudo ./target/debug/contained "$@"
