# Installation

Multiple ways to install and use acvp-cli.

## System Requirements

- **Operating System**: Linux, macOS, or Windows
- **Rust**: Version 1.70 or later
- **Memory**: 512 MB RAM minimum
- **Disk Space**: ~50 MB for build artifacts

## Installation Methods

### From Source

1. **Clone the repository**:

   ```bash
   git clone https://git.amongbytes.com/AmongBytes/acvp-cli
   cd acvp-cli
   ```

2. **Build**:

   ```bash
   cargo build --release
   ```

3. **Binary location**:

   ```bash
   ./target/release/acvp-cli
   ```

### Using Cargo Install

Install directly from the repository:

```bash
cargo install --git https://git.amongbytes.com/AmongBytes/acvp-cli \
  --root /usr/local \
  acvp-cli
```

Or from a local clone:

```bash
cd acvp-rust
cargo install --path . --root /usr/local
```

## Verifying the Installation

### Check Version

```bash
acvp-cli --version
```

Expected output:

```
acvp-cli 1.6.7
```

### Run Tests

```bash
cargo test
```

Expected output:

```
test result: ok. 47 passed; 0 failed; 0 ignored
```

## Wrapper Binaries

acvp-cli forwards cryptographic operations to an external wrapper binary over a binary protocol. Two options are available:

### Built-in ML-KEM Wrapper

`mlkem_wrapper` ships as part of acvp-cli. It implements all ML-KEM operations (FIPS 203) using the `mlkem-edu` Rust library and requires no external dependencies:

```bash
cargo build --release
# binary is at target/release/mlkem_wrapper
```

### C++ Modulewrapper

For the full algorithm suite backed by PQCryptoLib:

```bash
cd /path/to/modulewrapper-source
cmake -B build -S .
cmake --build build --target modulewrapper
# binary is at build/modulewrapper/modulewrapper
```
