.PHONY: build test clean

build:
	uv run --active maturin develop

test: build
	uv run --active pytest -q

clean:
	rm -rf target .pytest_cache .venv
