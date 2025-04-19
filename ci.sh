#!/bin/bash

set -euxo pipefail

cd embedded-resources
cargo b --features "_test"
cargo clippy --features "_test" -- --deny warnings

cd ../tests
cargo b
cargo clippy -- --deny warnings
cargo test
