"""Scrape Cargo lints allowed by default, one file per group, into ../cargo."""

import re
from html import unescape
from pathlib import Path
from urllib.request import urlopen

from toolchain import RUST

TARGET = Path(__file__).resolve().parents[1] / "cargo"
SOURCE = f"https://doc.rust-lang.org/{RUST}.0/cargo/reference/lints.html"
DOCS = "https://doc.rust-lang.org/stable/cargo/reference/lints.html"
ALLOWED = re.compile(r'<h2 id="allowed-by-default".*?<ul>(.*?)</ul>', re.DOTALL)
NAME = re.compile(r"<code>([a-z0-9_]+)</code>")
CODE = re.compile(r"</?code>")
TAG = re.compile(r"<[^>]+>")
GROUPS = ("pedantic", "restriction")
HEAD = """# Cargo {group}

Every lint of the Cargo `{group}` group that is allowed by default as of Rust {rust}.

| Lint | Description |
| --- | --- |
"""


def main() -> None:
    TARGET.mkdir(parents=True, exist_ok=True)
    for name, rows in grouped_rows(fetched()).items():
        head = HEAD.format(group=name, rust=RUST)
        (TARGET / f"{name}.md").write_text(head + "".join(rows))


def grouped_rows(page: str) -> dict[str, list[str]]:
    groups: dict[str, list[str]] = {}
    for lint in allowed_lints(page):
        rows = groups.setdefault(group(lint, page), [])
        rows.append(lint_row(lint, page))
    return groups


def fetched() -> str:
    with urlopen(SOURCE) as response:
        return response.read().decode()


def allowed_lints(page: str) -> list[str]:
    allowed = ALLOWED.search(page)
    if allowed is None:
        raise ValueError(f"No allowed-by-default list in {SOURCE}")
    return NAME.findall(allowed.group(1))


def group(lint: str, page: str) -> str:
    section = re.search(
        rf'<h2 id="{lint}".*?Group: <code>([a-z]+)</code>', page, re.DOTALL
    )
    if section is None:
        raise ValueError(f"No group of `{lint}` in {SOURCE}")
    name = section.group(1)
    if name not in GROUPS:
        raise ValueError(
            f"`{lint}` is in `{name}`, add it to GROUPS and tests/preset.rs"
        )
    return name


def lint_row(lint: str, page: str) -> str:
    return f"| [`{lint}`]({DOCS}#{lint}) | {summary(lint, page)} |\n"


def summary(lint: str, page: str) -> str:
    section = re.search(
        rf'<h2 id="{lint}".*?What it does.*?<p>(.*?)</p>', page, re.DOTALL
    )
    if section is None:
        raise ValueError(f"No description of `{lint}` in {SOURCE}")
    plain = TAG.sub("", CODE.sub("`", section.group(1)))
    return " ".join(unescape(plain).split())


main()
