const std = @import("std");

/// # DX Component Generator (Corrected)
///
/// This function is the core of our generator. It's exported with a C ABI,
/// so it can be called directly from our Rust observer.
///
/// It performs the following steps:
/// 1. Takes a file path as input.
/// 2. Reads the entire content of the file into memory.
/// 3. Loops through the content, searching for all instances of "<Dx.{IconName}>".
/// 4. For each one found, it extracts the {IconName}.
/// 5. It creates a new file named "{IconName}.tsx".
/// 6. It writes a boilerplate React component into the new file.
///
/// @param allocator The memory allocator to use for file operations.
/// @param file_path The path to the .tsx file to process.
/// @returns `true` if at least one component was generated, `false` otherwise or on error.
fn generate_component_from_file(allocator: std.mem.Allocator, file_path: []const u8) !bool {
    // Read the entire content of the source .tsx file.
    const file_content = try std.fs.cwd().readFileAlloc(allocator, file_path, 1_000_000); // Limit to 1MB
    defer allocator.free(file_content);

    const start_marker = "<Dx.";
    const end_marker = ">";
    var generated_something = false;
    var search_offset: usize = 0;

    // Loop to find all occurrences of the start marker.
    while (search_offset < file_content.len) {
        // Find the next occurrence of "<Dx." starting from our current offset.
        const found_start = std.mem.indexOf(u8, file_content[search_offset..], start_marker);

        if (found_start) |start_index| {
            // Adjust the index to be relative to the whole file_content buffer
            const absolute_start_index = search_offset + start_index + start_marker.len;

            // Now search for the closing ">" from this new position.
            const rest_of_content = file_content[absolute_start_index..];
            const found_end = std.mem.indexOf(u8, rest_of_content, end_marker);

            if (found_end) |end_index| {
                // We have the name!
                const icon_name = rest_of_content[0..end_index];

                if (icon_name.len > 0) {
                    // Create the new filename.
                    const new_filename = try std.fmt.allocPrint(allocator, "{s}.tsx", .{icon_name});
                    defer allocator.free(new_filename);

                    std.debug.print("Zig: Found component <Dx.{s}>. Generating file: {s}\n", .{ icon_name, new_filename });

                    // Create the new file.
                    const new_file = try std.fs.cwd().createFile(new_filename, .{});
                    defer new_file.close();

                    // Define the boilerplate content for the new component.
                    // All `{` and `}` characters for JSX must be escaped as `{{` and `}}`.
                    const component_template =
                        \\import React from 'react';
                        \\
                        \\const {s} = () => {{{{
                        \\  return (
                        \\    <div>
                        \\      {{/* TODO: Implement the {s} icon */}}
                        \\      <span>{{{s}}}</span>
                        \\    </div>
                        \\  );
                        \\}}}};
                        \\
                        \\export default {s};
                        \\
                    ;

                    // Write the boilerplate to the new file.
                    try new_file.writer().print(component_template, .{icon_name, icon_name, icon_name, icon_name});
                    generated_something = true;
                }

                // Update the search offset to continue searching after the current tag.
                search_offset = absolute_start_index + end_index;
            } else {
                // No closing tag found, stop searching.
                break;
            }
        } else {
            // No more start markers found, we're done.
            break;
        }
    }

    return generated_something;
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
    defer _ = gpa.deinit();

    // Use Zig's native slicing syntax to create the slice from the C pointer and length.
    const file_path = path_ptr[0..path_len];

    // We wrap the call in a `try` block. If `generate_component_from_file` returns
    // any error, the `catch` block will execute.
    const result = generate_component_from_file(allocator, file_path) catch |err| {
        std.debug.print("Zig Error: Could not process file '{s}': {s}\n", .{ file_path, @errorName(err) });
        return false;
    };

    return result;
}
