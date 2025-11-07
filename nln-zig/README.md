# nln - Zig Port

An idiomatic Zig port of the `nln` utility, which removes trailing newlines and carriage returns from stdin.

## Design Decisions

### Project Structure

**Single-file implementation**: Unlike the Rust version which separates library (`src/lib.rs`) and binary (`src/main.rs`), the Zig port consolidates everything into `src/main.zig`. This decision was made because:

- Zig's build system makes library/binary separation less critical
- The original separation in Rust was primarily for fuzzing support
- A single file is simpler and more maintainable for this small utility
- Tests are integrated inline using Zig's `test` blocks

### Memory Management

**Page allocator for newline buffer**: The core algorithm uses `std.heap.page_allocator` for the newline buffer (`nlbuf`). This choice reflects:

- **Simplicity**: No need to thread an allocator through the API
- **Performance**: The newline buffer is typically small (only trailing newlines)
- **Zero cost in common case**: Most inputs have minimal trailing newlines

**No allocations in hot path**: Like the Rust version, the algorithm processes data in chunks without allocating per-chunk, only accumulating potential trailing newlines.

### I/O Strategy

**Compatibility layer for Zig 0.15.x I/O changes**: Zig 0.15 introduced a major I/O redesign ("Writergate"). The implementation handles both old and new APIs:

```zig
const ReaderType = @TypeOf(reader);
const ActualType = if (@typeInfo(ReaderType) == .pointer)
    @typeInfo(ReaderType).pointer.child
else
    ReaderType;
const bytes_read = if (@hasDecl(ActualType, "readSliceShort"))
    try reader.readSliceShort(&buffer)
else
    try reader.read(&buffer);
```

This compile-time branching ensures compatibility while maintaining zero runtime overhead.

**Why this approach:**
- **Graceful API migration**: Works with both old generic readers (test mocks) and new `std.Io.Reader` interface
- **Zero cost abstraction**: The `if` is evaluated at compile time; only the relevant branch is compiled
- **Type introspection**: Uses Zig's metaprogramming to detect pointer types and extract the underlying struct
- **Future-proof**: When old APIs are fully removed, the compatibility code can be simplified without changing the algorithm

**Buffered I/O**: The `main()` function uses explicit buffering:
- **stdin**: 8192-byte buffer (matches algorithm's internal buffer)
- **stdout**: 4096-byte buffer with explicit flushing
- **stderr**: Unbuffered (for immediate error output)

This mirrors the Rust version's locked I/O approach but is more explicit due to Zig 0.15's new design.

### Algorithm Implementation

The core `snickerdoodle()` function is a direct port of the Rust algorithm with Zig idioms:

**Rust → Zig mappings:**
- `Vec<u8>` → `std.ArrayList(u8)`
- `BufRead::fill_buf()` → Reader iteration with `readSliceShort()`
- `Option<usize>` → `?usize`
- `Result<T, E>` → `!T` (error union)
- Generic traits → `anytype` parameters

**Use of `anytype` for reader and writer parameters:**

The function signature uses `anytype` for maximum flexibility:

```zig
pub fn snickerdoodle(reader: anytype, writer: anytype) !void
```

**Rationale:**
- **Compile-time polymorphism**: Zig generates specialized code for each reader/writer type at compile time, similar to Rust's monomorphization of generic traits
- **Zero runtime overhead**: No vtable indirection or dynamic dispatch
- **Test flexibility**: Allows tests to use simple mock types (like `fixedBufferStream`) without implementing formal interfaces
- **I/O API compatibility**: Works with both Zig 0.14's deprecated readers and 0.15's new `std.Io.Reader`/`std.Io.Writer` types
- **Idiomatic Zig**: Zig's standard library extensively uses `anytype` for I/O functions (see `std.io.Reader`, `std.io.Writer`)

**Type safety**: Despite using `anytype`, compile-time checks ensure:
- Reader must have a `read()` or `readSliceShort()` method
- Writer must have a `writeAll()` method
- Type mismatches are caught at compile time, not runtime

This is more flexible than Rust's trait-based approach while maintaining the same performance characteristics.

**Key algorithm properties preserved:**
- Single-pass streaming (no backtracking)
- Chunk-based processing
- Newline buffer for potential trailing bytes
- Byte-level operations (no UTF-8 overhead)

### CLI Design

**Minimal argument parsing**: Hand-rolled argument parsing instead of using a library:
- Only `--help`, `-h`, `--version`, `-v` supported
- Error on unknown arguments
- Simple linear scan with `std.process.argsWithAllocator()`

This keeps dependencies at zero (only stdlib) and binary size minimal.

**Version injection via build system**: Version and description are compile-time constants from `build.zig.zon`:

```zig
const options = b.addOptions();
options.addOption([]const u8, "version", "1.0.0");
options.addOption([]const u8, "description", "...");
```

This is more idiomatic in Zig than Rust's `env!()` macro approach.

### Testing Strategy

**Inline test blocks**: All 30+ tests from the Rust version ported as inline `test` blocks:

```zig
test "single LF trailing" {
    try testSnickerdoodle("abc\n", "abc");
}
```

**Custom test writer**: Since `std.ArrayList(u8)` doesn't directly implement the writer interface, tests use a lightweight wrapper:

```zig
const Writer = struct {
    list: *std.ArrayList(u8),
    allocator: std.mem.Allocator,

    pub fn writeAll(self: @This(), data: []const u8) !void {
        try self.list.appendSlice(self.allocator, data);
    }
};
```

This is simpler than Rust's generic `Vec<u8>` writer and makes the test boundary explicit.

**Invariant verification**: Every test verifies the critical invariant:

```zig
// Verify invariant: output never ends with \r or \n
if (output.items.len > 0) {
    const last = output.items[output.items.len - 1];
    try std.testing.expect(last != '\r');
    try std.testing.expect(last != '\n');
}
```

### Build Configuration

**Release optimization**: Mirrors Rust's aggressive optimization:

```zig
const exe_module = b.createModule(.{
    .root_source_file = b.path("src/main.zig"),
    .target = target,
    .optimize = optimize,
    .strip = if (optimize != .Debug) true else false,
});
```

- `ReleaseFast`: Equivalent to Rust's `release` profile
- Symbol stripping: Matches Rust's `strip = true`
- No LTO needed: Zig's compilation model already does whole-program optimization

### Mise Integration

**Namespaced tasks**: Zig tasks are prefixed with `zig:` to avoid conflicts:

```toml
[tasks."zig:build"]
description = "Build optimized Zig release binary"
run = "cd nln-zig && zig build -Doptimize=ReleaseFast"

[tasks."zig:test"]
description = "Run Zig tests"
run = "cd nln-zig && zig build test --summary all"
```

This allows both Rust and Zig versions to coexist in the same repository.

## Performance Characteristics

### Binary Size

- **Zig**: 26 KB (stripped, ReleaseFast)
- **Rust**: 334 KB (stripped, LTO, 1 codegen unit)
- **Ratio**: ~13x smaller

The size difference comes from:
- Zig's simpler runtime (no unwinding tables by default)
- More aggressive dead code elimination
- Smaller stdlib footprint

### Runtime Performance

Both implementations use the same algorithm and should have similar performance:
- Single-pass streaming
- Minimal allocations
- Efficient byte-level operations

## Building and Testing

### Prerequisites

```bash
# Zig is managed via mise
mise install
```

### Build

```bash
mise run zig:build        # Release build
mise run zig:test         # Run tests
mise run zig:fmt          # Format code
mise run zig:fmt:check    # Check formatting
mise run zig:ci           # Run all CI checks
```

### Run

```bash
mise run zig:run          # Run the binary
# or directly:
./zig-out/bin/nln < input.txt
```

### Install

```bash
mise run zig:install      # Install to ~/.local/bin
```

## Zig Version

This port targets **Zig 0.15.2** and accounts for the major I/O redesign introduced in 0.15.x.

## Differences from Rust Version

| Aspect | Rust | Zig |
|--------|------|-----|
| **Structure** | Separate lib + bin | Single file |
| **Error handling** | `Result<T, E>` | `!T` error unions |
| **Memory** | Explicit allocator args | Page allocator |
| **I/O** | Trait-based | Interface-based (0.15+) |
| **Testing** | External test file | Inline test blocks |
| **Binary size** | 334 KB | 26 KB |
| **Dependencies** | Zero (stdlib only) | Zero (stdlib only) |

## License

MIT (same as parent project)
