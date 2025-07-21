// Import the standard library, which contains all the essential tools we need.
const std = @import("std");

// The main function is the entry point of our program.
// It can return an error, which is why we have `!void`.
pub fn main() !void {
    // We'll use a general-purpose allocator for managing memory.
    // This is a robust choice for most applications.
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    // `defer` ensures that we check for memory leaks when the function exits.
    // It's a great feature for writing safe and correct programs.
    defer {
        const deinit_status = gpa.deinit();
        if (deinit_status == .leak) {
            std.log.warn("Memory leak detected!", .{});
        }
    }

    // Define the directory and the content for the files.
    const dir_name = "zig_modules";
    const file_count = 100;
    const content = "Hello, manfromexistence!";

    // Ensure the target directory exists. If not, create it.
    // This makes the program runnable without manual setup.
    const cwd = std.fs.cwd();
    try cwd.makeDir(dir_name);

    // Let's start the clock!
    // We create a high-precision timer to measure the operation's duration.
    var timer = try std.time.Timer.start();

    // This is the main loop where the magic happens.
    // We'll loop `file_count` times to create each file.
    var i: u32 = 0;
    while (i < file_count) : (i += 1) {
        // Dynamically create the filename for each iteration (e.g., "file_0.txt", "file_1.txt").
        // We allocate memory for the filename string.
        const filename = try std.fmt.allocPrint(allocator, "file_{d}.txt", .{i});
        // `defer` here ensures the allocated memory for the filename is freed at the end of each loop iteration.
        defer allocator.free(filename);

        // Open (or create) the file within the specified directory.
        // We're using `cwd.createFile` which is a convenient way to create a file in a relative path.
        const file = try cwd.createFile(.{ .sub_path = dir_name, .basename = filename }, .{});
        // And of course, we `defer` the closing of the file to ensure it's always released.
        defer file.close();

        // Write the content to the file.
        // `writeAll` ensures that the entire string is written.
        try file.writer().writeAll(content);
    }

    // Stop the clock!
    // The `read()` function gives us the elapsed time in nanoseconds.
    const elapsed_nanos = timer.read();
    // We convert nanoseconds to milliseconds for a more human-readable output.
    const elapsed_ms = @as(f64, @floatFromInt(elapsed_nanos)) / 1_000_000.0;

    // Print the results to the console.
    std.debug.print("\nSuccessfully created {d} files in the '{s}' directory.\n", .{ file_count, dir_name });
    std.debug.print("Total time taken: {d:.4}ms\n\n", .{elapsed_ms});
}

