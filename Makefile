.PHONY: test

test:
	pytest --doctest-modules
	cargo test --manifest-path core/Cargo.toml
