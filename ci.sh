#!/bin/bash

set -euxo pipefail

cd tests
cargo test
