"""Scrape rustc lints allowed by default and edition migration groups into ../rustc."""

import re
import subprocess
from itertools import starmap
from pathlib import Path
from tempfile import TemporaryDirectory

from toolchain import RUST

TARGET = Path(__file__).resolve().parents[1] / "rustc"
DOCS = f"https://doc.rust-lang.org/{RUST}.0/rustc/lints/listing/allowed-by-default.html"
GROUPS_DOCS = f"https://doc.rust-lang.org/{RUST}.0/rustc/lints/groups.html"
ALLOWED_LINT = re.compile(r"^\s+([a-z0-9-]+)\s+allow\s+(.*)$", re.MULTILINE)
MIGRATION_GROUP = re.compile(r"^\s+(rust-\d+-compatibility)\s+(.*)$", re.MULTILINE)
UNSTABLE_LINT = re.compile(r"the `([a-z0-9_]+)` lint is unstable")
HEAD = f"""# rustc allowed by default

Every stable rustc lint that is allowed by default as of Rust {RUST}, besides edition migrations that `cargo fix --edition` turns on.

| Lint | Description |
| --- | --- |
"""
GROUPS_HEAD = """
## Edition migrations

| Group | Lints |
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


def underscored(lint: str) -> str:
    return lint.replace("-", "_")


def group_row(group: str, members: str) -> str:
    names = map(underscored, members.split(", "))
    lints = ", ".join(map("`{0}`".format, names))
    return f"| [`{underscored(group)}`]({GROUPS_DOCS}) | {lints} |\n"


def lint_row(anchor: str, summary: str) -> str:
    return f"| [`{underscored(anchor)}`]({DOCS}#{anchor}) | {summary} |\n"


def lints_table(lints: str, migrations: list[tuple[str, str]]) -> str:
    allowed = ALLOWED_LINT.findall(lints)
    migrated = {lint for _, members in migrations for lint in members.split(", ")}
    skipped = migrated | unstable(allowed)
    rows = [lint_row(*lint) for lint in allowed if lint[0] not in skipped]
    return HEAD + "".join(rows)


def main() -> None:
    lints, groups = rustc("-W", "help").stdout.split("Lint groups provided by rustc:")
    migrations = MIGRATION_GROUP.findall(groups)
    group_rows = "".join(starmap(group_row, migrations))
    TARGET.mkdir(parents=True, exist_ok=True)
    (TARGET / "allowed-by-default.md").write_text(
        lints_table(lints, migrations) + GROUPS_HEAD + group_rows
    )


main()
