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

    // Get the current working directory.
    const cwd = std.fs.cwd();
    // Ensure the target directory exists. `makePath` is idempotent, meaning
    // it won't error if the directory already exists, making the program robust.
    try cwd.makePath(dir_name);

    // Let's start the clock!
    // We create a high-precision timer to measure the operation's duration.
    var timer = try std.time.Timer.start();

    // This is the main loop where the magic happens.
    // We'll loop `file_count` times to create each file.
    var i: u32 = 0;
    while (i < file_count) : (i += 1) {
        // Dynamically create the full filepath for each iteration (e.g., "zig_modules/file_0.txt").
        // We format the string and allocate memory for it in one step.
        const filepath = try std.fmt.allocPrint(allocator, "{s}/file_{d}.txt", .{ dir_name, i });
        // `defer` ensures the allocated memory for the filepath is freed at the end of each loop iteration.
        defer allocator.free(filepath);

        // Open (or create) the file using the full path relative to the current working directory.
        // The second argument is for options; we pass an empty struct for defaults.
        const file = try cwd.createFile(filepath, .{});
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
