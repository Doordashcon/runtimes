#!/bin/bash
set -e

# Change to the runtimes directory
cd /app/runtimes

# Display help
show_help() {
  echo "Ambassador Governance Pallet Docker Helper"
  echo ""
  echo "Available commands:"
  echo "  help                    - Show this help message"
  echo "  test                    - Run tests with output visible"
  echo "  build                   - Build the pallet"
  echo "  check                   - Check the pallet with runtime benchmarks feature"
  echo "  benchmark               - Run benchmarks for the pallet"
  echo "  lint                    - Run linting checks"
  echo "  format                  - Format the code"
  echo "  all                     - Run all tests, build, and benchmarks"
  echo ""
  echo "Example: docker run -v \$(pwd):/app/runtimes ambassador-governance test"
}

# Run tests
run_tests() {
  echo "Running tests with output visible..."
  cargo test -p pallet-ambassador-governance -- --nocapture
}

# Function to build the pallet
build_pallet() {
  echo "Building the pallet..."
  cargo build --release -p pallet-ambassador-governance
}

# Check with runtime benchmarks
check_benchmarks() {
  echo "Checking with runtime benchmarks feature..."
  cargo check --features runtime-benchmarks -p pallet-ambassador-governance
}

# Build with runtime benchmarks
build_benchmarks() {
  echo "Building with runtime benchmarks feature..."
  cargo build --release --features runtime-benchmarks
}

# Run benchmarks
run_benchmarks() {
  echo "Running benchmarks..."
  check_benchmarks
  build_benchmarks
  
  echo "Measuring extrinsic weights..."
  RUST_LOG=debug,trie_cache=warn frame-omni-bencher v1 benchmark pallet \
    --runtime ./target/release/wbuild/collectives-polkadot-runtime/collectives_polkadot_runtime.wasm \
    --pallet pallet_ambassador_governance \
    --extrinsic "*" \
    --template /app/frame-weight-template.hbs \
    --output ./pallets/ambassador-governance/src/weights.rs > ./log.txt 2>&1
  
  echo "Benchmark results saved to log.txt"
}

# Run linting
run_lint() {
  echo "Running linting checks..."
  cargo fmt -p pallet-ambassador-governance --check
}

# Format code
format_code() {
  echo "Formatting code..."
  rustup default nightly
  cargo fmt -p pallet-ambassador-governance
  rustup default stable
}

# Run all checks
run_all() {
  run_tests
  build_pallet
  run_benchmarks
  run_lint
}

# Main command handler
case "$1" in
  help)
    show_help
    ;;
  test)
    run_tests
    ;;
  build)
    build_pallet
    ;;
  check)
    check_benchmarks
    ;;
  benchmark)
    run_benchmarks
    ;;
  lint)
    run_lint
    ;;
  format)
    format_code
    ;;
  all)
    run_all
    ;;
  *)
    show_help
    exit 1
    ;;
esac
