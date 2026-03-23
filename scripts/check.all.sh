#!/usr/bin/env bash
set -euo pipefail

cargo hack --manifest-path ../Cargo.toml build --each-feature