# Copyright (c) 2024-2025, DeciSym, LLC
# Licensed under either of:
# - Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
# - BSD 3-Clause License (https://opensource.org/licenses/BSD-3-Clause)
# at your option.

lint:
	cargo install  cargo-machete
	cargo fmt --check
	cargo machete
	cargo clippy --benches --tests --bins --no-deps --all-features

build:
	cargo build

test:
	cargo test

presubmit: lint test
