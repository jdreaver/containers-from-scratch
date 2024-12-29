#!/usr/bin/env bash

set -euo pipefail

cd rust/
exec cargo run contained -- "$@"
