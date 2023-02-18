#!/bin/bash

set -ue

cd "$(git rev-parse --show-toplevel)/main"
rust-bundler . | ~/.cargo/bin/rustfmt --edition=2018 | pbcopy
