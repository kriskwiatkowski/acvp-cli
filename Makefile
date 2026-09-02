# Makefile for acvp-cli

.PHONY: all build release debug test clean check fmt clippy help install

# Default target
all: release

# Build release version (optimized)
release:
	@echo "Building release version..."
	cargo build --release
	@echo "Binary: target/release/acvp-cli"

# Build debug version
debug:
	@echo "Building debug version..."
	cargo build
	@echo "Binary: target/debug/acvp-cli"

# Alias for release
build: release

# Run tests
test:
	cargo test

# Run checks without building
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Run clippy linter
clippy:
	cargo clippy -- -D warnings

# Clean build artifacts
clean:
	cargo clean
	rm -f capabilities.json
	rm -rf responses/

# Install to system (requires sudo)
install: release
	@echo "Installing to /usr/local/bin..."
	sudo install -m 755 target/release/acvp-cli /usr/local/bin/

# Quick test with modulewrapper (requires parent build)
test-regcap: release
	@echo "Testing regcap..."
	./target/release/acvp-cli --wrapper ../build/modulewrapper/modulewrapper --regcap

# Show help
help:
	@echo "acvp-cli Makefile"
	@echo ""
	@echo "Targets:"
	@echo "  all        - Build release version (default)"
	@echo "  build      - Build release version"
	@echo "  release    - Build optimized release version"
	@echo "  debug      - Build debug version"
	@echo "  test       - Run Rust tests"
	@echo "  check      - Check code without building"
	@echo "  fmt        - Format code with rustfmt"
	@echo "  clippy     - Run clippy linter"
	@echo "  clean      - Clean build artifacts"
	@echo "  install    - Install to /usr/local/bin (requires sudo)"
	@echo "  help       - Show this help"
