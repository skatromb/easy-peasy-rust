"""Scrape rustdoc lints allowed by default into ../rustdoc."""

import re
import subprocess
from pathlib import Path
from tempfile import TemporaryDirectory

from toolchain import RUST

TARGET = Path(__file__).resolve().parents[1] / "rustdoc"
DOCS = "https://doc.rust-lang.org/stable/rustdoc/lints.html"
ALLOWED_LINT = re.compile(r"^\s+rustdoc::([a-z0-9-]+)\s+allow\s+(.*)$", re.MULTILINE)
UNSTABLE_LINT = re.compile(r"the `rustdoc::([a-z0-9_]+)` lint is unstable")
HEAD = f"""# rustdoc allowed by default

Every stable rustdoc lint that is allowed by default as of Rust {RUST}.

| Lint | Description |
| --- | --- |
"""


def main() -> None:
    allowed = allowed_lints()
    skipped = unstable(allowed)
    rows = [lint_row(*lint) for lint in allowed if lint[0] not in skipped]
    TARGET.mkdir(parents=True, exist_ok=True)
    (TARGET / "allowed-by-default.md").write_text(HEAD + "".join(rows))


def allowed_lints() -> list[tuple[str, str]]:
    lints = ALLOWED_LINT.findall(rustdoc("-W", "help").stdout)
    return [(underscored(lint), summary) for lint, summary in lints]


def unstable(lints: list[tuple[str, str]]) -> set[str]:
    flags = [f"-Wrustdoc::{lint}" for lint, _ in lints]
    with TemporaryDirectory() as out:
        source = Path(out) / "lib.rs"
        source.write_text("")
        probe = rustdoc("--crate-type=lib", f"--out-dir={out}", *flags, str(source))
    return set(UNSTABLE_LINT.findall(probe.stderr))


def rustdoc(*args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["rustdoc", *args],
        capture_output=True,
        text=True,
        check=True,
    )


def underscored(lint: str) -> str:
    return lint.replace("-", "_")


def lint_row(lint: str, summary: str) -> str:
    return f"| [`{lint}`]({DOCS}#{lint}) | {summary} |\n"


main()
