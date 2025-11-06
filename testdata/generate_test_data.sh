#!/bin/bash
# Generate test data files for benchmarking nnl

set -e

# Generate files in the directory this script is contained in
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "Generating test data files in $SCRIPT_DIR..."

# 1. Small file with no trailing newlines
echo -n "Hello World" > "$SCRIPT_DIR/small_no_trailing.txt"

# 2. Small file with many trailing newlines
{
  echo -n "Hello World"
  printf '\n%.0s' {1..1000}
} > "$SCRIPT_DIR/small_many_trailing.txt"

# 3. Medium file with content, newlines in middle, content, no trailing
{
  echo -n "Start of file"
  printf '\n%.0s' {1..500}
  echo -n "Middle content here"
  printf '\n%.0s' {1..500}
  echo -n "End of file"
} > "$SCRIPT_DIR/medium_no_trailing.txt"

# 4. Medium file with content, newlines in middle, content, many trailing
{
  echo -n "Start of file"
  printf '\n%.0s' {1..500}
  echo -n "Middle content here"
  printf '\n%.0s' {1..500}
  echo -n "End of file"
  printf '\n%.0s' {1..2000}
} > "$SCRIPT_DIR/medium_many_trailing.txt"

# 5. Small file with many newlines then content at the end (no trailing)
{
  echo -n "Hello World"
  printf '\n%.0s' {1..1000}
  echo -n "Final content"
} > "$SCRIPT_DIR/small_many_trailing_then_content.txt"

# 6. Medium file with content, many newlines, then final content
{
  echo -n "Start of file"
  printf '\n%.0s' {1..500}
  echo -n "Middle content here"
  printf '\n%.0s' {1..2000}
  echo -n "End content"
} > "$SCRIPT_DIR/medium_many_trailing_then_content.txt"

# 7. File with only newlines (edge case)
printf '\n%.0s' {1..5000} > "$SCRIPT_DIR/only_newlines.txt"

# 8. Large file with realistic content (~500KB)
{
  for i in {1..10000}; do
    echo "This is line $i with some content to make it longer"
    if [ $((i % 1000)) -eq 0 ]; then
      # Add chunks of newlines every 1000 lines
      printf '\n%.0s' {1..100}
    fi
  done
  # No trailing newline at the end
  echo -n "Final line"
} > "$SCRIPT_DIR/large_no_trailing.txt"

# 9. Large file with many trailing newlines (~500KB)
{
  for i in {1..10000}; do
    echo "This is line $i with some content to make it longer"
    if [ $((i % 1000)) -eq 0 ]; then
      printf '\n%.0s' {1..100}
    fi
  done
  echo -n "Final line"
  # Add 10000 trailing newlines
  printf '\n%.0s' {1..10000}
} > "$SCRIPT_DIR/large_many_trailing.txt"

# 10. Large file with many newlines then final content
{
  for i in {1..10000}; do
    echo "This is line $i with some content to make it longer"
    if [ $((i % 1000)) -eq 0 ]; then
      printf '\n%.0s' {1..100}
    fi
  done
  echo -n "Middle line"
  # Add 10000 newlines then final content
  printf '\n%.0s' {1..10000}
  echo -n "Final content at end"
} > "$SCRIPT_DIR/large_many_trailing_then_content.txt"

# 11. Mixed line endings (CRLF and LF)
{
  printf "Line 1\r\n"
  printf "Line 2\n"
  printf "Line 3\r\n"
  printf '\n%.0s' {1..100}
  printf "Line 4\n"
  printf '\r\n%.0s' {1..100}
} > "$SCRIPT_DIR/mixed_line_endings.txt"

# 12. Very large file (~1GB) - worst case with massive trailing newlines
{
  for i in {1..15000000}; do
    echo "Line $i: Lorem ipsum dolor sit amet, consectetur adipiscing elit"
  done
  echo -n "Last line"
  printf '\n%.0s' {1..1000000}
} > "$SCRIPT_DIR/huge_many_trailing.txt"

# 13. Empty file
touch "$SCRIPT_DIR/empty.txt"

echo "Test data files generated!"
echo ""
echo "File sizes:"
ls -lh "$SCRIPT_DIR" | grep -v "^d" | grep -v "^total"
