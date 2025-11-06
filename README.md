# nln

Remove trailing newlines and carriage returns from stdin.

## Installation

```sh
cargo install nln
```

Or download from [releases](https://github.com/glennib/nln/releases).

## Usage

```sh
# Remove trailing newlines
echo -e "hello\n\n" | nln

# Use with files
cat file.txt | nln

# Copy to clipboard without trailing newlines
cat script.sh | nln | pbcopy
```

The tool preserves newlines within content, only removing trailing ones.

## License

MIT
