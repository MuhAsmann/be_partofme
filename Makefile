# Nama binary dari Cargo.toml
BIN_NAME=rust_api

# Database URL dan logging
ENV_VARS=RUST_LOG=info,actix_web=info,tracing_actix_web=info

# Default: compile dan run
run:
	$(ENV_VARS) cargo run

# Build release
build:
	cargo build --release

# Clean build artifacts
clean:
	cargo clean

# Check linting
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Format source code
fmt:
	cargo fmt --all

# Run tests
test:
	cargo test

# Run diesel migration (pastikan diesel-cli sudah terinstal)
migration-run:
	diesel migration run

migration-revert:
	diesel migration revert

migration-generate:
	@read -p "Enter migration name: " name; \
	diesel migration generate $$name

# Watch project and run (requires cargo-watch)
watch:
	$(ENV_VARS) cargo watch -x run

.PHONY: run build clean lint fmt test migration-run migration-revert migration-generate watch
