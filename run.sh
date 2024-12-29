#!/usr/bin/env bash

set -euo pipefail

cd rust/
cargo run contained -- "$@"
