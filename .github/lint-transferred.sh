#!/usr/bin/env bash
set -euo pipefail

toolchain_file=$1
counts=$2

cp "$toolchain_file" transferred/rust-toolchain.toml
cd transferred
rustup toolchain install

cargo clippy \
  --workspace \
  --all-targets \
  --message-format=json \
  -- --cap-lints warn \
  | jq -r 'select(.reason == "compiler-message") | .message.code.code // empty' \
  | sort \
  | uniq -c \
  > "../$counts"
