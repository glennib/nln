# nln

Remove trailing newlines and carriage returns from stdin.

## Installation

Using cargo:

```sh
cargo install nln
```

Using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) (faster, no compilation):

```sh
cargo binstall nln
```

Using [mise](https://mise.jdx.dev/) (global):

```sh
mise use -g cargo:nln
```

Or download binaries from [releases](https://github.com/glennib/nln/releases).

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
