#!/bin/bash
# Benchmark nnl with various test data files

set -e

# Build the project first
echo "Building nnl in release mode..."
cargo build --release

# Get the actual binary path from cargo
BINARY=$(cargo build --release --message-format=json 2>/dev/null | jq -r 'select(.executable != null) | .executable' | grep nnl | head -1)
TESTDATA_DIR="testdata"
BENCHMARKS_DIR="benchmarks"

if [ ! -d "$TESTDATA_DIR" ]; then
  echo "Test data directory not found. Run ./testdata/generate_test_data.sh first."
  exit 1
fi

# Create benchmarks directory
mkdir -p "$BENCHMARKS_DIR"

echo ""
echo "Running benchmarks..."
echo "====================="
echo ""

# Benchmark each test file
for file in "$TESTDATA_DIR"/*.txt; do
  filename=$(basename "$file")
  echo "Benchmarking: $filename"
  hyperfine \
    --warmup 3 \
    --runs 10 \
    --export-markdown "$BENCHMARKS_DIR/${filename%.txt}.md" \
    "$BINARY < $file > /dev/null"
  echo ""
done

echo "Benchmark complete! Results saved to $BENCHMARKS_DIR/"
