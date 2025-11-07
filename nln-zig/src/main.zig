const std = @import("std");
const build_options = @import("build_options");

/// Removes trailing newlines and carriage returns from input.
///
/// This function uses a buffered streaming approach with a "newline buffer" to handle
/// trailing newlines in a single pass without backtracking:
///
/// 1. Reads input in chunks using buffered I/O
/// 2. Maintains a newline buffer (nlbuf) to track potential trailing newlines
/// 3. Only outputs newlines when followed by actual content
/// 4. Performs single-pass processing
///
/// Algorithm:
/// - For each chunk of input:
///   - Find the last non-newline character
///   - If chunk contains only newlines: append to nlbuf and continue
///   - If chunk contains content:
///     - Flush accumulated nlbuf to output (these weren't trailing)
///     - Write bytes up to and including last non-newline
///     - Put remaining newlines into nlbuf
///
/// Both '\r' (CR) and '\n' (LF) are considered newlines.
pub fn snickerdoodle(reader: anytype, writer: anytype) !void {
    var nlbuf: std.ArrayList(u8) = .empty;
    defer nlbuf.deinit(std.heap.page_allocator);

    // Read in chunks
    var buffer: [8192]u8 = undefined;
    while (true) {
        // Support both old and new I/O APIs
        const ReaderType = @TypeOf(reader);
        const ActualType = if (@typeInfo(ReaderType) == .pointer)
            @typeInfo(ReaderType).pointer.child
        else
            ReaderType;
        const bytes_read = if (@hasDecl(ActualType, "readSliceShort"))
            try reader.readSliceShort(&buffer)
        else
            try reader.read(&buffer);
        if (bytes_read == 0) break; // EOF

        const chunk = buffer[0..bytes_read];

        // Find last non-newline character
        var last_non_nl: ?usize = null;
        var i: usize = chunk.len;
        while (i > 0) {
            i -= 1;
            if (chunk[i] != '\n' and chunk[i] != '\r') {
                last_non_nl = i;
                break;
            }
        }

        if (last_non_nl) |pos| {
            // We have actual content in this chunk
            // Flush the newline buffer (these weren't trailing after all)
            if (nlbuf.items.len > 0) {
                try writer.writeAll(nlbuf.items);
                nlbuf.clearRetainingCapacity();
            }

            // Write up to and including the last non-newline character
            try writer.writeAll(chunk[0 .. pos + 1]);

            // Save any trailing newlines to nlbuf
            if (pos + 1 < chunk.len) {
                try nlbuf.appendSlice(std.heap.page_allocator, chunk[pos + 1 ..]);
            }
        } else {
            // Only newlines in this chunk - add to nlbuf
            try nlbuf.appendSlice(std.heap.page_allocator, chunk);
        }
    }
}

/// Print help message (marked cold for optimization)
fn printHelp(writer: anytype) !void {
    try writer.print(
        \\nln {s}
        \\{s}
        \\
        \\USAGE:
        \\    nln [OPTIONS]
        \\
        \\OPTIONS:
        \\    -h, --help       Print help information
        \\    -v, --version    Print version information
        \\
        \\Reads from stdin and writes to stdout, removing trailing newlines and carriage returns.
        \\
    , .{ build_options.version, build_options.description });
}

/// Print version (marked cold for optimization)
fn printVersion(writer: anytype) !void {
    try writer.print("{s}\n", .{build_options.version});
}

/// Print error message to stderr (marked cold for optimization)
fn printError(stderr: anytype, arg: []const u8) !void {
    try stderr.print("error: unexpected argument '{s}'\n\n", .{arg});
    try stderr.writeAll("Usage: nln [OPTIONS]\n\nFor more information, try '--help'.\n");
}

pub fn main() !void {
    // Set up buffered stdout
    var stdout_buffer: [4096]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    // Set up unbuffered stderr (for error messages)
    var stderr_writer = std.fs.File.stderr().writer(&.{});
    const stderr = &stderr_writer.interface;

    // Set up buffered stdin
    var stdin_buffer: [8192]u8 = undefined;
    var stdin_reader = std.fs.File.stdin().reader(&stdin_buffer);
    const stdin = &stdin_reader.interface;

    // Simple argument parsing
    var args = try std.process.argsWithAllocator(std.heap.page_allocator);
    defer args.deinit();

    _ = args.skip(); // Skip program name

    if (args.next()) |arg| {
        if (std.mem.eql(u8, arg, "--help") or std.mem.eql(u8, arg, "-h")) {
            try printHelp(stdout);
            try stdout.flush();
            return;
        } else if (std.mem.eql(u8, arg, "--version") or std.mem.eql(u8, arg, "-v")) {
            try printVersion(stdout);
            try stdout.flush();
            return;
        } else {
            try printError(stderr, arg);
            try stderr.flush();
            std.process.exit(1);
        }
    }

    // Normal operation: process stdin to stdout
    try snickerdoodle(stdin, stdout);
    try stdout.flush();
}

// ============================================================================
// Tests
// ============================================================================

test "empty input" {
    try testSnickerdoodle("", "");
}

test "no newlines" {
    try testSnickerdoodle("abc", "abc");
}

test "single LF trailing" {
    try testSnickerdoodle("abc\n", "abc");
}

test "single CRLF trailing" {
    try testSnickerdoodle("abc\r\n", "abc");
}

test "single CR trailing" {
    try testSnickerdoodle("abc\r", "abc");
}

test "multiple LF trailing" {
    try testSnickerdoodle("abc\n\n\n", "abc");
}

test "multiple CRLF trailing" {
    try testSnickerdoodle("abc\r\n\r\n", "abc");
}

test "multiple CR trailing" {
    try testSnickerdoodle("abc\r\r\r", "abc");
}

test "mixed trailing newlines 1" {
    try testSnickerdoodle("abc\n\r\n", "abc");
}

test "mixed trailing newlines 2" {
    try testSnickerdoodle("abc\r\n\n\r", "abc");
}

test "only LF" {
    try testSnickerdoodle("\n", "");
}

test "only multiple LF" {
    try testSnickerdoodle("\n\n\n", "");
}

test "only CRLF" {
    try testSnickerdoodle("\r\n\r\n", "");
}

test "only CR" {
    try testSnickerdoodle("\r\r\r", "");
}

test "leading LF preserved" {
    try testSnickerdoodle("\nabc", "\nabc");
}

test "leading multiple LF preserved" {
    try testSnickerdoodle("\n\nabc", "\n\nabc");
}

test "leading CRLF preserved" {
    try testSnickerdoodle("\r\nabc", "\r\nabc");
}

test "leading and trailing" {
    try testSnickerdoodle("\nabc\n", "\nabc");
}

test "middle newlines preserved" {
    try testSnickerdoodle("ab\nc\n", "ab\nc");
}

test "multiple middle newlines preserved" {
    try testSnickerdoodle("ab\n\nc\n", "ab\n\nc");
}

test "mixed newlines in content" {
    try testSnickerdoodle("a\rb\nc\r\nd\n", "a\rb\nc\r\nd");
}

test "large input with trailing" {
    const allocator = std.testing.allocator;
    var input: std.ArrayList(u8) = .empty;
    defer input.deinit(allocator);

    // 100,000 'a' characters followed by newlines
    try input.appendNTimes(allocator, 'a', 100_000);
    try input.appendSlice(allocator, "\n\n\n");

    var expected: std.ArrayList(u8) = .empty;
    defer expected.deinit(allocator);
    try expected.appendNTimes(allocator, 'a', 100_000);

    try testSnickerdoodle(input.items, expected.items);
}

test "large input with middle and trailing newlines" {
    const allocator = std.testing.allocator;
    var input: std.ArrayList(u8) = .empty;
    defer input.deinit(allocator);

    // 50,000 'a' + newlines + 50,000 'b' + trailing newlines
    try input.appendNTimes(allocator, 'a', 50_000);
    try input.appendSlice(allocator, "\n\n");
    try input.appendNTimes(allocator, 'b', 50_000);
    try input.appendSlice(allocator, "\n\n\n");

    var expected: std.ArrayList(u8) = .empty;
    defer expected.deinit(allocator);
    try expected.appendNTimes(allocator, 'a', 50_000);
    try expected.appendSlice(allocator, "\n\n");
    try expected.appendNTimes(allocator, 'b', 50_000);

    try testSnickerdoodle(input.items, expected.items);
}

test "CRLF variations" {
    try testSnickerdoodle("line1\r\nline2\r\n", "line1\r\nline2");
}

test "mixed line endings" {
    try testSnickerdoodle("line1\r\nline2\nline3\r", "line1\r\nline2\nline3");
}

test "only spaces no newlines" {
    try testSnickerdoodle("   ", "   ");
}

test "trailing space then newline" {
    try testSnickerdoodle("abc \n", "abc ");
}

test "alternating content and newlines" {
    try testSnickerdoodle("a\nb\nc\n", "a\nb\nc");
}

test "CRLF then LF" {
    try testSnickerdoodle("abc\r\n\n", "abc");
}

test "LF then CRLF" {
    try testSnickerdoodle("abc\n\r\n", "abc");
}

// Helper function for tests
fn testSnickerdoodle(input: []const u8, expected: []const u8) !void {
    var input_stream = std.io.fixedBufferStream(input);
    var output: std.ArrayList(u8) = .empty;
    defer output.deinit(std.testing.allocator);

    // Create a simple wrapper for ArrayList writer
    const Writer = struct {
        list: *std.ArrayList(u8),
        allocator: std.mem.Allocator,

        pub fn writeAll(self: @This(), data: []const u8) !void {
            try self.list.appendSlice(self.allocator, data);
        }
    };

    const writer = Writer{ .list = &output, .allocator = std.testing.allocator };
    try snickerdoodle(input_stream.reader(), writer);

    try std.testing.expectEqualSlices(u8, expected, output.items);

    // Verify invariant: output never ends with \r or \n
    if (output.items.len > 0) {
        const last = output.items[output.items.len - 1];
        try std.testing.expect(last != '\r');
        try std.testing.expect(last != '\n');
    }
}
