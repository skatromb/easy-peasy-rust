"""Scrape the clippy groups the preset turns on into ../clippy, with preset levels."""

import re
import subprocess
import tomllib
from pathlib import Path
from tempfile import TemporaryDirectory

REPO = "https://github.com/rust-lang/rust-clippy"
TAG = "rust-1.99.0"
CLONE = (
    "git",
    "-c",
    "advice.detachedHead=false",
    "clone",
    "-q",
    "--depth=1",
    f"--branch={TAG}",
)
RULES = Path(__file__).resolve().parents[2]
DOCS = f"https://rust-lang.github.io/rust-clippy/{TAG}/index.html"
DECLARATION = re.compile(
    r'pub (\w+),(?:\s*//.*)*\s*(\w+),\s*(?:r#"(.*?)"#|"((?:[^"\\]|\\[\s\S])*)")'
)
ESCAPE = re.compile(r"\\(.)", re.DOTALL)
GROUPS = ("pedantic", "nursery", "restriction")
Lint = tuple[str, str, str]
HEAD = """# clippy {0}

Every lint of the clippy `{0}` group as of Rust 1.99, with its level in the preset.

| Lint | Preset | Description |
| --- | --- | --- |
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


def preset_level(preset: dict[str, object], group: str, lint: str) -> str:
    level = preset.get(lint, preset.get(group, "allow"))
    if isinstance(level, dict):
        return str(level["level"])
    return str(level)


def lint_row(preset: dict[str, object], group: str, name: str, summary: str) -> str:
    level = preset_level(preset, group, name)
    return f"| [`{name}`]({DOCS}#{name}) | {level} | {summary} |\n"


def group_file(group: str, preset: dict[str, object], lints: list[Lint]) -> str:
    rows = [
        lint_row(preset, group, name, summary)
        for lint_group, name, summary in lints
        if lint_group == group
    ]
    return HEAD.format(group) + "".join(rows)


def write_groups(source: Path) -> None:
    preset = tomllib.loads((RULES / "lints.toml").read_text())["clippy"]
    lints = declared_lints(source)
    target = RULES / "external" / "clippy"
    target.mkdir(parents=True, exist_ok=True)
    for group in GROUPS:
        (target / f"{group}.md").write_text(group_file(group, preset, lints))


def main() -> None:
    with TemporaryDirectory() as clone:
        subprocess.run([*CLONE, REPO, clone], check=True)
        write_groups(Path(clone) / "clippy_lints" / "src")


main()
