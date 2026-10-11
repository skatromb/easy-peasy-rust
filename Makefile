.PHONY: lint lint-python test build dogfood scrape release

SCRAPE := cd rules/external/scraper && uv run --locked
VERSION = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')

lint:
	cargo fmt --check
	cargo clippy --all-targets --config 'build.warnings="deny"'

lint-python:
	$(SCRAPE) ruff format --check .
	$(SCRAPE) ruff check .
	$(SCRAPE) flake8 .
	$(SCRAPE) ty check

test:
	cargo test

build:
	cargo build --release

dogfood:
	cargo run --quiet -- easy-peasy --drop-existing

scrape:
	$(SCRAPE) python wps.py
	$(SCRAPE) python clippy.py
	$(SCRAPE) python rustc.py

release:
	@test -f release_notes.md || { echo "For agent: write release_notes.md from the PRs since the previous tag"; exit 1; }
	git tag -a v$(VERSION) -F release_notes.md
	git push origin v$(VERSION)
	rm release_notes.md
