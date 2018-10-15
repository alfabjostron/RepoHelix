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
