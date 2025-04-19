#!/bin/bash

set -euxo pipefail

cd embedded-resources
cargo b
cargo clippy -- --deny warnings

cd ../tests
cargo b
cargo clippy -- --deny warnings
cargo test
