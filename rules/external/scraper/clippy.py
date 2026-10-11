"""Scrape the clippy groups that are allowed by default and the settings into ../clippy."""

import re
import subprocess
from itertools import starmap
from pathlib import Path
from tempfile import TemporaryDirectory

from toolchain import RUST

REPO = "https://github.com/rust-lang/rust-clippy"
TAG = f"rust-{RUST}.0"
CLONE = (
    "git",
    "-c",
    "advice.detachedHead=false",
    "clone",
    "-q",
    "--depth=1",
    f"--branch={TAG}",
)
TARGET = Path(__file__).resolve().parents[1] / "clippy"
DOCS = f"https://rust-lang.github.io/rust-clippy/{TAG}/index.html"
LINT = re.compile(
    r"^\s+clippy::([a-z0-9-]+)\s+(?:allow|warn|deny)\s+(.*)$", re.MULTILINE
)
GROUP = re.compile(r"^\s+clippy::(\w+)\s+(clippy::.*)$", re.MULTILINE)
GROUPS = ("cargo", "pedantic", "nursery", "restriction")
HEAD = """# clippy {group}

Every lint of the clippy `{group}` group as of Rust {rust}. All are allowed by default.

| Lint | Description |
| --- | --- |
"""
SETTINGS_DOCS = "https://doc.rust-lang.org/clippy/lint_configuration.html"
SETTING = re.compile(
    r"^## `([a-z0-9-]+)`\n(.*?)\n\n.*?^\*\*Default Value:\*\* `([^\n]*)`$"
    r".*?^\*\*Affected lints:\*\*\n((?:\* [^\n]*\n?)+)",
    re.MULTILINE | re.DOTALL,
)
AFFECTED_LINT = re.compile(r"\[`(\w+)`\]")
SETTINGS_HEAD = f"""# clippy settings

Every `clippy.toml` setting as of Rust {RUST}.

| Setting | Default | Lints | Description |
| --- | --- | --- | --- |
"""


def underscored(lint: str) -> str:
    return lint.removeprefix("clippy::").replace("-", "_")


def group_file(group: str, members: str, summaries: dict[str, str]) -> str:
    rows = [
        f"| [`{name}`]({DOCS}#{name}) | {summaries[name]} |\n"
        for name in sorted(map(underscored, members.split(", ")))
    ]
    return HEAD.format(group=group, rust=RUST) + "".join(rows)


def lint_summaries(lints: str) -> dict[str, str]:
    return {underscored(name): summary for name, summary in LINT.findall(lints)}


def write_groups() -> None:
    lints, groups = subprocess.run(
        ["clippy-driver", "-W", "help"], capture_output=True, text=True, check=True
    ).stdout.split("Lint groups loaded by this crate:")
    summaries = lint_summaries(lints)
    members = dict(GROUP.findall(groups))
    TARGET.mkdir(parents=True, exist_ok=True)
    for group in GROUPS:
        (TARGET / f"{group}.md").write_text(
            group_file(group, members[group], summaries)
        )


def setting_row(name: str, description: str, default: str, affected: str) -> str:
    lints = ", ".join(map("`{0}`".format, AFFECTED_LINT.findall(affected)))
    summary = " ".join(description.split())
    link = f"[`{name}`]({SETTINGS_DOCS}#{name})"
    return f"| {link} | `{default}` | {lints} | {summary} |\n"


def write_settings(book: Path) -> None:
    settings = SETTING.findall((book / "lint_configuration.md").read_text())
    rows = starmap(setting_row, settings)
    (TARGET / "settings.md").write_text(SETTINGS_HEAD + "".join(rows))


def main() -> None:
    write_groups()
    with TemporaryDirectory() as clone:
        subprocess.run([*CLONE, REPO, clone], check=True)
        write_settings(Path(clone) / "book" / "src")


main()
