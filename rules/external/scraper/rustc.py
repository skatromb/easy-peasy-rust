"""Scrape rustc lints allowed by default into ../rustc, with preset levels."""

import re
import subprocess
import tomllib
from pathlib import Path

RULES = Path(__file__).resolve().parents[2]
DOCS = "https://doc.rust-lang.org/1.99.0/rustc/lints/listing/allowed-by-default.html"
ALLOWED_LINT = re.compile(r"^\s+([a-z0-9-]+)\s+allow\s+(.*)$", re.MULTILINE)
HEAD = """# rustc allowed by default

Every rustc lint that is allowed by default as of Rust 1.99, with its level in the preset.

| Lint | Preset | Description |
| --- | --- | --- |
"""


def lint_row(preset: dict[str, str], anchor: str, summary: str) -> str:
    name = anchor.replace("-", "_")
    level = preset.get(name, "allow")
    return f"| [`{name}`]({DOCS}#{anchor}) | {level} | {summary} |\n"


def main() -> None:
    preset = tomllib.loads((RULES / "lints.toml").read_text())["rust"]
    help_text = subprocess.run(
        ["rustc", "-W", "help"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout
    lints = help_text.split("Lint groups provided by rustc:")[0]
    rows = [lint_row(preset, *lint) for lint in ALLOWED_LINT.findall(lints)]
    target = RULES / "external" / "rustc"
    target.mkdir(parents=True, exist_ok=True)
    (target / "allowed-by-default.md").write_text(HEAD + "".join(rows))


main()
