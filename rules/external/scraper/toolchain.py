"""The Rust release from ../../../rust-toolchain.toml."""

import tomllib
from pathlib import Path

TOOLCHAIN = Path(__file__).resolve().parents[3] / "rust-toolchain.toml"
RUST = tomllib.loads(TOOLCHAIN.read_text())["toolchain"]["channel"]
