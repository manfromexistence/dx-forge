const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const data_dir = "ui/data/icons";
    var total_icon_count: u32 = 0;

    std.debug.print("Transformer: Scanning directory '{s}' for icon sets...\n", .{data_dir});

    var dir = try std.fs.cwd().openDir(data_dir, .{});
    defer dir.close();

    var iterator = dir.iterate();
    while (try iterator.next()) |entry| {
        if (!std.mem.endsWith(u8, entry.name, ".json")) {
            continue;
        }

        const file_path = try std.fs.path.join(allocator, &.{ data_dir, entry.name });
        defer allocator.free(file_path);

        std.debug.print("  -> Processing: {s}\n", .{file_path});

        const json_data = try std.fs.cwd().readFileAlloc(allocator, file_path, 10 * 1024 * 1024); // 10MB limit
        defer allocator.free(json_data);

        // --- CORRECTED: Parse into a generic JSON Value (DOM) first ---
        var json_dom = try std.json.parseFromSlice(std.json.Value, allocator, json_data, .{});
        defer json_dom.deinit();

        // --- 3. Navigate the DOM to find the icon count ---
        // Get the top-level object.
        const root_object = json_dom.value.object;

        // Get the "icons" object from within the top-level object.
        const icons_value = root_object.get("icons") orelse {
            std.debug.print("  -> WARNING: Skipping file {s}, no 'icons' field found.\n", .{file_path});
            continue;
        };
        
        const icons_object = icons_value.object;
        const count_in_file = icons_object.count();
        total_icon_count += @intCast(count_in_file);
    }

    std.debug.print("\nTransformer: Found a total of {d} icons across all files.\n", .{total_icon_count});

    const binary_file = try std.fs.cwd().createFile("icons.bin", .{});
    defer binary_file.close();
    const writer = binary_file.writer();

    try writer.writeInt(u32, total_icon_count, .little);

    std.debug.print("Transformer: Successfully created icons.bin.\n", .{});
}
