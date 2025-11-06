#!/bin/bash
# Benchmark nnl with various test data files
# Usage: ./benchmark.sh BIN [BASELINE_BIN]

set -e

# Parse arguments
if [ $# -lt 1 ]; then
  echo "Usage: $0 BIN [BASELINE_BIN]"
  echo ""
  echo "  BIN           Path to the binary to benchmark (required)"
  echo "  BASELINE_BIN  Path to baseline binary for comparison (optional)"
  exit 1
fi

BINARY="$1"
BASELINE_BINARY="$2"
TESTDATA_DIR="testdata"
BENCHMARKS_DIR="benchmarks"

# Validate primary binary
if [ ! -f "$BINARY" ]; then
  echo "Error: Binary not found at $BINARY"
  exit 1
fi

if [ ! -x "$BINARY" ]; then
  echo "Error: $BINARY is not executable"
  exit 1
fi

# Validate baseline binary if provided
if [ -n "$BASELINE_BINARY" ]; then
  if [ ! -f "$BASELINE_BINARY" ]; then
    echo "Error: Baseline binary not found at $BASELINE_BINARY"
    exit 1
  fi

  if [ ! -x "$BASELINE_BINARY" ]; then
    echo "Error: $BASELINE_BINARY is not executable"
    exit 1
  fi
fi

if [ ! -d "$TESTDATA_DIR" ]; then
  echo "Test data directory not found. Run ./testdata/generate_test_data.sh first."
  exit 1
fi

# Create benchmarks directory
mkdir -p "$BENCHMARKS_DIR"

echo ""
echo "Running benchmarks..."
echo "====================="
echo "Binary: $BINARY"
if [ -n "$BASELINE_BINARY" ]; then
  echo "Baseline: $BASELINE_BINARY"
fi
echo ""

# Benchmark each test file
for file in "$TESTDATA_DIR"/*.txt; do
  filename=$(basename "$file")
  echo "Benchmarking: $filename"

  if [ -n "$BASELINE_BINARY" ]; then
    # Compare current vs baseline
    hyperfine \
      --warmup 3 \
      --runs 10 \
      --export-markdown "$BENCHMARKS_DIR/${filename%.txt}.md" \
      --command-name "current" "$BINARY < $file > /dev/null" \
      --command-name "baseline" "$BASELINE_BINARY < $file > /dev/null"
  else
    # Benchmark single binary
    hyperfine \
      --warmup 3 \
      --runs 10 \
      --export-markdown "$BENCHMARKS_DIR/${filename%.txt}.md" \
      "$BINARY < $file > /dev/null"
  fi

  echo ""
done

echo "Benchmark complete! Results saved to $BENCHMARKS_DIR/"
