#!/bin/bash
# Local CI simulation script - runs the same checks as CI

set -e

echo "🔍 Running local CI checks..."
echo "=============================="
echo ""

cd "$(dirname "$0")/.."

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

FAILED=0

# Function to run a check
run_check() {
    local name=$1
    shift
    echo -e "${YELLOW}▶ $name${NC}"
    if "$@"; then
        echo -e "${GREEN}✓ $name passed${NC}"
        echo ""
        return 0
    else
        echo -e "${RED}✗ $name failed${NC}"
        echo ""
        FAILED=1
        return 1
    fi
}

# 1. Format check
run_check "Format Check" cargo fmt --all -- --check

# 2. Clippy
run_check "Clippy" cargo clippy --all-features -- -D warnings

# 3. Tests
run_check "Tests" cargo test --all-features

# 4. Build (release)
run_check "Build (release)" cargo build --release

# 5. Binary verification
run_check "Binary Check" bash -c './target/release/acvp-cli --version'

echo "=============================="
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ All checks passed!${NC}"
    echo ""
    echo "Binary size:"
    ls -lh target/release/acvp-cli | awk '{print $5, $9}'
    echo ""
    echo "You're ready to push!"
    exit 0
else
    echo -e "${RED}✗ Some checks failed${NC}"
    echo ""
    echo "Please fix the issues before pushing."
    exit 1
fi
