.PHONY: lint lint-python test test-old build dogfood scrape release

SCRAPE := cd rules/external/scraper && uv run --locked
VERSION = $(shell cargo pkgid | sed 's/.*@//')

lint:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings

lint-python:
	$(SCRAPE) ruff format --check .
	$(SCRAPE) ruff check .
	$(SCRAPE) flake8 .
	$(SCRAPE) ty check

test:
	cargo test

test-old:
	rustup toolchain install 1.50 1.74 --profile minimal --component clippy
	cargo test -- --ignored

build:
	cargo build --release

dogfood:
	cargo run --quiet -- easy-peasy --yes --drop-existing

scrape:
	$(SCRAPE) python wps.py
	$(SCRAPE) python clippy.py
	$(SCRAPE) python rustc.py

release:
	git tag -a v$(VERSION) -m v$(VERSION)
	git push origin v$(VERSION)
