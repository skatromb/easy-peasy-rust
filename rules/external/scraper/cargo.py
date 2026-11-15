"""Scrape Cargo lints allowed by default into ../cargo."""

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
HEAD = f"""# Cargo allowed by default

Every Cargo lint that is allowed by default as of Rust {RUST}.

| Lint | Description |
| --- | --- |
"""


def main() -> None:
    page = fetched()
    rows = [lint_row(lint, page) for lint in allowed_lints(page)]
    TARGET.mkdir(parents=True, exist_ok=True)
    (TARGET / "allowed-by-default.md").write_text(HEAD + "".join(rows))


def fetched() -> str:
    with urlopen(SOURCE) as response:
        return response.read().decode()


def allowed_lints(page: str) -> list[str]:
    allowed = ALLOWED.search(page)
    if allowed is None:
        raise ValueError(f"No allowed-by-default list in {SOURCE}")
    return NAME.findall(allowed.group(1))


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
