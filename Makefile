# repohelix — developer tasks.
#
# The Rust core needs only cargo. The viewer targets need Node + npm, but the
# core Rust targets never depend on them, so `make build test` works on a
# machine without Node installed.

CARGO ?= cargo
