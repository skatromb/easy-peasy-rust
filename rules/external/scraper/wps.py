"""Scrape every wemake-python-styleguide violation into ../wps, one file per group."""

import inspect
from importlib import import_module
from importlib.metadata import version
from operator import attrgetter
from pathlib import Path

from wemake_python_styleguide.violations.base import BaseViolation

VERSION = version("wemake-python-styleguide")
PACKAGE = "wemake_python_styleguide.violations"
TARGET = Path(__file__).resolve().parents[1] / "wps"
DOCS = (
    "https://wemake-python-styleguide.readthedocs.io/en/latest/pages/usage/violations/"
)
GROUPS = (
    ("system", "000-099", "System"),
    ("naming", "100-199", "Naming"),
    ("complexity", "200-299", "Complexity"),
    ("consistency", "300-399", "Consistency"),
    ("best_practices", "400-499", "Best practices"),
    ("refactoring", "500-599", "Refactoring"),
    ("oop", "600-699", "OOP"),
)
HEAD = """# WPS {1} {2}

Every violation in the {2} group of wemake-python-styleguide {4}, from its [docs]({0}{3}.html).
Disabled ones are struck through.

| Code | Violation | Description |
| --- | --- | --- |
"""


def summary(violation: type[BaseViolation]) -> str:
    docstring = (inspect.getdoc(violation) or "").removeprefix(
        f"{violation.full_code} — "
    )
    first_paragraph = docstring.split("\n\n")[0].replace("``", "`")
    return " ".join(first_paragraph.split())


def violation_link(module: str, name: str, disabled: str | None) -> str:
    anchor = f"{module}.html#{PACKAGE}.{module}.{name}"
    link = f"[{name}]({DOCS}{anchor})"
    return f"~~{link}~~ disabled since {disabled}" if disabled else link


def violation_row(module: str, violation: type[BaseViolation]) -> str:
    link = violation_link(module, violation.__name__, violation.disabled_since)
    number = str(violation.code).zfill(3)
    return f"| WPS{number} | {link} | {summary(violation)} |\n"


def group_rows(module: str) -> list[str]:
    namespace = import_module(f"{PACKAGE}.{module}")
    violations = (
        member
        for _, member in inspect.getmembers(namespace, inspect.isclass)
        if issubclass(member, BaseViolation) and member.__module__ == namespace.__name__
    )
    ordered = sorted(violations, key=attrgetter("code"))
    return [violation_row(module, violation) for violation in ordered]


def main() -> None:
    TARGET.mkdir(parents=True, exist_ok=True)
    for module, codes, title in GROUPS:
        head = HEAD.format(DOCS, codes, title, module, VERSION)
        target = TARGET / f"{codes}-{module.replace('_', '-')}.md"
        target.write_text(head + "".join(group_rows(module)))


main()
