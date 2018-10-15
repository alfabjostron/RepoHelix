# repohelix — developer tasks.
#
# The Rust core needs only cargo. The viewer targets need Node + npm, but the
# core Rust targets never depend on them, so `make build test` works on a
# machine without Node installed.

CARGO ?= cargo
FIXTURE ?= fixtures/nebula.gitlog

.DEFAULT_GOAL := help

.PHONY: help
help: ## Show this help
	@echo "repohelix make targets:"
	@echo "  build         Build the release binary"
	@echo "  test          Run Rust unit + integration tests"
	@echo "  fmt           Format Rust sources"
	@echo "  fmt-check     Verify formatting"
	@echo "  clippy        Lint with clippy (if installed)"
	@echo "  demo          Print the text report for the bundled fixture"
	@echo "  demo-json     Print the JSON report for the bundled fixture"
	@echo "  viewer-build  Build the TypeScript viewer"
	@echo "  viewer-test   Run the TypeScript core tests"
	@echo "  viewer-data   Regenerate viewer/data.js from the fixture"
	@echo "  check         fmt-check + test (+ viewer-test if Node present)"
	@echo "  clean         Remove build artifacts"

.PHONY: build
build: ## Build the release binary
	$(CARGO) build --release

.PHONY: test
test: ## Run Rust tests
	$(CARGO) test

.PHONY: fmt
fmt: ## Format Rust sources
	$(CARGO) fmt

.PHONY: fmt-check
fmt-check: ## Verify Rust formatting
	$(CARGO) fmt --check

