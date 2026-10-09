"""Scrape rustc lints allowed by default into ../rustc."""

import re
import subprocess
from pathlib import Path

from toolchain import RUST

TARGET = Path(__file__).resolve().parents[1] / "rustc"
DOCS = f"https://doc.rust-lang.org/{RUST}.0/rustc/lints/listing/allowed-by-default.html"
ALLOWED_LINT = re.compile(r"^\s+([a-z0-9-]+)\s+allow\s+(.*)$", re.MULTILINE)
HEAD = f"""# rustc allowed by default

Every rustc lint that is allowed by default as of Rust {RUST}.

| Lint | Description |
| --- | --- |
"""


def lint_row(anchor: str, summary: str) -> str:
    name = anchor.replace("-", "_")
    return f"| [`{name}`]({DOCS}#{anchor}) | {summary} |\n"


def main() -> None:
    help_text = subprocess.run(
        ["rustc", "-W", "help"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout
    lints = help_text.split("Lint groups provided by rustc:")[0]
    rows = [lint_row(*lint) for lint in ALLOWED_LINT.findall(lints)]
    TARGET.mkdir(parents=True, exist_ok=True)
    (TARGET / "allowed-by-default.md").write_text(HEAD + "".join(rows))


main()
