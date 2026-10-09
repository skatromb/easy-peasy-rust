"""Scrape the clippy groups that are allowed by default into ../clippy."""

import re
import subprocess
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
DECLARATION = re.compile(
    r'pub (\w+),(?:\s*//.*)*\s*(\w+),\s*(?:r#"(.*?)"#|"((?:[^"\\]|\\[\s\S])*)")'
)
ESCAPE = re.compile(r"\\(.)", re.DOTALL)
GROUPS = ("pedantic", "nursery", "restriction")
Lint = tuple[str, str, str]
HEAD = """# clippy {group}

Every lint of the clippy `{group}` group as of Rust {rust}. All are allowed by default.

| Lint | Description |
| --- | --- |
"""


def lint_entry(declaration: tuple[str, str, str, str]) -> Lint:
    name, group, raw_summary, summary = declaration
    words = (raw_summary or ESCAPE.sub(r"\1", summary)).split()
    return group, name.lower(), " ".join(words)


def declared_lints(source: Path) -> list[Lint]:
    declarations = (
        declaration
        for path in source.rglob("*.rs")
        for declaration in DECLARATION.findall(path.read_text())
    )
    return sorted(map(lint_entry, declarations))


def group_file(group: str, lints: list[Lint]) -> str:
    rows = [
        f"| [`{name}`]({DOCS}#{name}) | {summary} |\n"
        for lint_group, name, summary in lints
        if lint_group == group
    ]
    return HEAD.format(group=group, rust=RUST) + "".join(rows)


def write_groups(source: Path) -> None:
    lints = declared_lints(source)
    TARGET.mkdir(parents=True, exist_ok=True)
    for group in GROUPS:
        (TARGET / f"{group}.md").write_text(group_file(group, lints))


def main() -> None:
    with TemporaryDirectory() as clone:
        subprocess.run([*CLONE, REPO, clone], check=True)
        write_groups(Path(clone) / "clippy_lints" / "src")


main()
