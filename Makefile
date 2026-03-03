.PHONY: test test-python test-rust

test: test-python test-rust

test-python:
	. python/.venv/bin/activate && pytest -q

test-rust:
	cd rust/qc_core && cargo test
