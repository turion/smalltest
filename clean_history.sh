#!/bin/sh

set -e

git fetch origin master

git rebase origin/master --exec "cargo fmt -- --check && cargo clippy --all-targets --all-features -- -D warnings"
