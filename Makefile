.PHONY: lint lint-python test build scrape release

SCRAPE := cd rules/external/scraper && uv run --locked
VERSION = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')

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

release:
	git tag -a v$(VERSION) -m v$(VERSION)
	git push origin v$(VERSION)
