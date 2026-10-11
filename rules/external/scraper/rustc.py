"""Scrape rustc lints allowed by default into ../rustc."""

import re
import subprocess
from pathlib import Path
from tempfile import TemporaryDirectory

from toolchain import RUST

TARGET = Path(__file__).resolve().parents[1] / "rustc"
DOCS = f"https://doc.rust-lang.org/{RUST}.0/rustc/lints/listing/allowed-by-default.html"
ALLOWED_LINT = re.compile(r"^\s+([a-z0-9-]+)\s+allow\s+(.*)$", re.MULTILINE)
MIGRATION_GROUP = re.compile(r"^\s+rust-\d+-compatibility\s+(.*)$", re.MULTILINE)
UNSTABLE_LINT = re.compile(r"the `([a-z0-9_]+)` lint is unstable")
HEAD = f"""# rustc allowed by default

Every stable rustc lint that is allowed by default as of Rust {RUST}, besides edition migrations that `cargo fix --edition` turns on.

| Lint | Description |
| --- | --- |
"""


def rustc(*args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["rustc", *args],
        input="",
        capture_output=True,
        text=True,
        check=True,
    )


def unstable(lints: list[tuple[str, str]]) -> set[str]:
    flags = [f"-W{anchor}" for anchor, _ in lints]
    with TemporaryDirectory() as out:
        probe = rustc(
            "--crate-type=lib", "--emit=metadata", f"--out-dir={out}", *flags, "-"
        )
    return {lint.replace("_", "-") for lint in UNSTABLE_LINT.findall(probe.stderr)}


def migrations(groups: str) -> set[str]:
    return {
        lint
        for members in MIGRATION_GROUP.findall(groups)
        for lint in members.split(", ")
    }


def lint_row(anchor: str, summary: str) -> str:
    name = anchor.replace("-", "_")
    return f"| [`{name}`]({DOCS}#{anchor}) | {summary} |\n"


def main() -> None:
    lints, groups = rustc("-W", "help").stdout.split("Lint groups provided by rustc:")
    allowed = ALLOWED_LINT.findall(lints)
    skipped = migrations(groups) | unstable(allowed)
    rows = [lint_row(*lint) for lint in allowed if lint[0] not in skipped]
    TARGET.mkdir(parents=True, exist_ok=True)
    (TARGET / "allowed-by-default.md").write_text(HEAD + "".join(rows))


main()
