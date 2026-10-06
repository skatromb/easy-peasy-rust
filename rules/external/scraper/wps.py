"""Scrape every wemake-python-styleguide violation into ../wps, one file per group."""

import ast
import subprocess
from pathlib import Path
from tempfile import TemporaryDirectory

REPO = "https://github.com/wemake-services/wemake-python-styleguide"
TAG = "1.8.1"
CLONE = (
    "git",
    "-c",
    "advice.detachedHead=false",
    "clone",
    "-q",
    "--depth=1",
    f"--branch={TAG}",
)
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


def class_constants(klass: ast.ClassDef) -> dict[str, object]:
    return {
        ast.unparse(stmt.targets[0]): stmt.value.value
        for stmt in klass.body
        if isinstance(stmt, ast.Assign) and isinstance(stmt.value, ast.Constant)
    }


def summary(klass: ast.ClassDef) -> str:
    docstring = ast.get_docstring(klass) or ""
    first_paragraph = docstring.split("\n\n")[0].replace("``", "`")
    return " ".join(first_paragraph.split())


def violation_link(module: str, name: str, disabled: object) -> str:
    anchor = f"{module}.html#wemake_python_styleguide.violations.{module}.{name}"
    link = f"[{name}]({DOCS}{anchor})"
    return f"~~{link}~~ disabled since {disabled}" if disabled else link


def violation_row(module: str, klass: ast.ClassDef) -> tuple[int, str] | None:
    constants = class_constants(klass)
    code = constants.get("code")
    if not isinstance(code, int):
        return None
    link = violation_link(module, klass.name, constants.get("disabled_since"))
    description = summary(klass)
    number = str(code).zfill(3)
    return code, f"| WPS{number} | {link} | {description} |\n"


def group_rows(source: Path, module: str) -> list[str]:
    tree = ast.parse((source / f"{module}.py").read_text())
    classes = (node for node in tree.body if isinstance(node, ast.ClassDef))
    rows = sorted(filter(None, (violation_row(module, klass) for klass in classes)))
    return [line for _, line in rows]


def write_groups(source: Path) -> None:
    TARGET.mkdir(parents=True, exist_ok=True)
    for module, codes, title in GROUPS:
        head = HEAD.format(DOCS, codes, title, module, TAG)
        target = TARGET / f"{codes}-{module.replace('_', '-')}.md"
        target.write_text(head + "".join(group_rows(source, module)))


def main() -> None:
    with TemporaryDirectory() as clone:
        subprocess.run([*CLONE, REPO, clone], check=True)
        write_groups(Path(clone) / "wemake_python_styleguide" / "violations")


main()
