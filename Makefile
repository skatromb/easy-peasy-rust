.PHONY: lint lint-python test build scrape

SCRAPE := cd rules/external/scraper && uv run --locked

lint: lint-python
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings

lint-python:
	$(SCRAPE) ruff format --check .
	$(SCRAPE) ruff check .
	$(SCRAPE) flake8 .
	$(SCRAPE) ty check

test:
	cargo test

build:
	cargo build --release

scrape:
	$(SCRAPE) python wps.py
	$(SCRAPE) python clippy.py
	$(SCRAPE) python rustc.py
