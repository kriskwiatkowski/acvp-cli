# Building from Source

Complete guide to building acvp-cli from source code.

## Prerequisites

### Required

1. **Rust** 1.70 or later

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Git**

   ```bash
   # Ubuntu/Debian
   sudo apt-get install git

   # macOS
   brew install git
   ```

### Optional

- **CMake** and **C++ compiler** (for modulewrapper)
- **Docker** (for containerized builds)

## Quick Build

### 1. Clone Repository

```bash
git clone https://git.amongbytes.com/AmongBytes/acvp-cli
cd acvp-cli
```

### 2. Build

```bash
cargo build --release
```

Binary location: `target/release/acvp-cli`

### 3. Verify

```bash
./target/release/acvp-cli --version
```

## Build Options

### Debug Build

Faster compilation, larger binary, includes debug symbols:

```bash
cargo build
```

Binary: `target/debug/acvp-cli`

### Release Build

Optimized, smaller binary:

```bash
cargo build --release
```

Binary: `target/release/acvp-cli`

### With Specific Features

```bash
cargo build --release --all-features
```

### Target-Specific Build

```bash
# Linux musl (static)
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl

# macOS ARM
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Windows
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

## Using Make

### Build Release

```bash
make
# or
make release
```

### Build Debug

```bash
make debug
```

### Run Tests

```bash
make test
```

### Clean

```bash
make clean
```

### Install System-Wide

```bash
make install
```

Installs to `/usr/local/bin/acvp-cli` (requires sudo).

## Build Script

Use the provided build script:

```bash
./build.sh
```

This script:

1. Builds release version
2. Shows binary location
3. Provides usage examples

## Docker Build

### Build Image

```bash
cd acvp-rust
docker build -t acvp-cli:local .
```

### Multi-Stage Build

Create optimized image:

```dockerfile
FROM rust:latest as builder
WORKDIR /build
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/acvp-cli /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/acvp-cli"]
```

## Cross-Compilation

### Setup Cross

Install `cross` for easy cross-compilation:

```bash
cargo install cross
```

### Build for Different Targets

```bash
# Linux ARM64
cross build --release --target aarch64-unknown-linux-gnu

# Windows
cross build --release --target x86_64-pc-windows-gnu

# Linux musl
cross build --release --target x86_64-unknown-linux-musl
```

## Optimization

### Profile-Guided Optimization (PGO)

```bash
# Build instrumented binary
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" \
  cargo build --release

# Run with representative workload
./target/release/acvp-cli --wrapper modulewrapper --regcap

# Build optimized binary
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" \
  cargo build --release
```

### Link-Time Optimization (LTO)

Add to `Cargo.toml`:

```toml
[profile.release]
lto = true
codegen-units = 1
```

Then build:

```bash
cargo build --release
```

### Size Optimization

For smallest binary:

```toml
[profile.release]
opt-level = 'z'
lto = true
codegen-units = 1
panic = 'abort'
strip = true
```

## Building Modulewrapper

acvp-cli requires the modulewrapper binary:

```bash
cd /path/to/modulewrapper-source
cmake -B build -S . -DCMAKE_BUILD_TYPE=Release
cmake --build build --target modulewrapper
```

Binary location: `build/modulewrapper/modulewrapper`

## Troubleshooting

### "linker `cc` not found"

Install C compiler:

```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# Fedora/RHEL
sudo dnf install gcc

# macOS
xcode-select --install
```

### "OpenSSL not found"

Install OpenSSL development files:

```bash
# Ubuntu/Debian
sudo apt-get install libssl-dev pkg-config

# Fedora/RHEL
sudo dnf install openssl-devel

# macOS
brew install openssl
```

### Out of Memory

Reduce parallelism:

```bash
cargo build --release -j 2
```

### Slow Build

Use sccache for caching:

```bash
cargo install sccache
export RUSTC_WRAPPER=sccache
cargo build --release
```

### Link Errors

Try static linking:

```bash
RUSTFLAGS='-C target-feature=+crt-static' \
  cargo build --release --target x86_64-unknown-linux-gnu
```

## Build Verification

### Check Binary

```bash
file target/release/acvp-cli
ldd target/release/acvp-cli  # Linux only
```

### Run Tests

```bash
cargo test --release
```

### Check Binary Size

```bash
ls -lh target/release/acvp-cli
```

Typical sizes:

- Debug: ~15-20 MB
- Release: ~7-10 MB
- Release (stripped): ~5-7 MB

### Strip Binary

Reduce size:

```bash
strip target/release/acvp-cli
```

## Development Build

For faster iteration:

```bash
# Use cargo-watch for auto-rebuild
cargo install cargo-watch
cargo watch -x check -x test

# Use sccache
export RUSTC_WRAPPER=sccache
cargo build
```
