.PHONY: test test-python test-rust

test: test-python test-rust

test-python:
	cd python && test -x .venv/bin/python || (echo "Missing python/.venv. Create it first." && exit 1)
	cd python && .venv/bin/python -m pytest -q

test-rust:
	cd rust/qc_core && cargo test
