const std = @import("std");

/// # DX Component Generator
///
/// This function is the core of our generator. It's exported with a C ABI,
/// so it can be called directly from our Rust observer.
///
/// It performs the following steps:
/// 1. Takes a file path as input.
/// 2. Reads the entire content of the file.
/// 3. Searches for the pattern "<Dx.{IconName}>".
/// 4. If found, it extracts the {IconName}.
/// 5. It creates a new file named "{IconName}.tsx".
/// 6. It writes a boilerplate React component into the new file.
///
/// @param allocator The memory allocator to use for file operations.
/// @param file_path The path to the .tsx file to process.
/// @returns `true` if a component was generated, `false` otherwise or on error.
fn generate_component_from_file(allocator: std.mem.Allocator, file_path: []const u8) !bool {
    // Read the entire content of the source .tsx file.
    const file_content = try std.fs.cwd().readFileAlloc(allocator, file_path, 1_000_000); // Limit to 1MB
    defer allocator.free(file_content);

    // Define the start and end markers we're looking for.
    const start_marker = "<Dx.";
    const end_marker = ">";

    // Find the position of our start marker.
    var stream = std.io.fixedBufferStream(file_content);
    const reader = stream.reader();
    
    while (try reader.readUntilDelimiterOrEof(start_marker.ptr, start_marker.len)) |found_line| {
        // We found "<Dx.", now we need to find the closing ">" to get the component name.
        if (std.mem.indexOf(u8, found_line, end_marker)) |end_pos| {
            const icon_name = found_line[0..end_pos];

            // We have the name! Let's create the new filename.
            const new_filename = try std.fmt.allocPrint(allocator, "{s}.tsx", .{icon_name});
            defer allocator.free(new_filename);

            std.debug.print("Zig: Found component <Dx.{s}>. Generating file: {s}\n", .{ icon_name, new_filename });

            // Create the new file.
            const new_file = try std.fs.cwd().createFile(new_filename, .{});
            defer new_file.close();

            // Define the boilerplate content for the new component.
            const component_template =
                \\import React from 'react';
                \\
                \\const {s} = () => {{
                \\  return (
                \\    <div>
                \\      {/* TODO: Implement the {s} icon */}
                \\      <span>{s}</span>
                \\    </div>
                \\  );
                \\}};
                \\
                \\export default {s};
                \\
            ;

            // Write the boilerplate to the new file.
            try new_file.writer().print(component_template, .{icon_name, icon_name, icon_name, icon_name});
            
            // We successfully generated a file, so we can return true.
            return true;
        }
    }


    // If we get here, no component was found.
    return false;
}

/// # FFI Export Function
///
/// This is the C-compatible wrapper function that Rust will call.
/// It sets up an allocator and calls our main Zig logic, handling errors gracefully.
///
/// @param path_ptr A pointer to the start of the file path string.
/// @param path_len The length of the file path string.
/// @returns `true` on success, `false` on failure.
export fn process_tsx_file(path_ptr: [*c]const u8, path_len: usize) bool {
    // It's crucial to have an allocator for memory management (e.g., reading files).
    // The page_allocator is a good general-purpose choice.
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const file_path = std.mem.sliceTo(path_ptr, path_len);

    // We wrap the call in a `try` block. If `generate_component_from_file` returns
    // any error, the `catch` block will execute.
    const result = generate_component_from_file(allocator, file_path) catch |err| {
        std.debug.print("Zig Error: Could not process file '{s}': {s}\n", .{ file_path, @errorName(err) });
        return false;
    };

    return result;
}
