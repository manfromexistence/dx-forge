// Import the standard build system tools from the Zig standard library.
const std = @import("std");

// The `build` function is the main entry point for the Zig build system.
pub fn build(b: *std.Build) void {
    // Get the target architecture and OS for cross-compilation.
    const target = b.standardTargetOptions(.{});

    // Get the optimization mode (Debug, ReleaseSafe, etc.).
    const optimize = b.standardOptimizeOption(.{});

    // --- Create the Executable ---
    // We define an executable artifact but notice we are NOT using .root_source_file.
    const exe = b.addExecutable(.{
        .name = "c_file_creator",
        .target = target,
        .optimize = optimize,
    });

    // --- Add C Source File (The Fix!) ---
    // This is the key change. We explicitly add `main.c` as a C source file.
    // This tells Zig to use its C compiler frontend for this specific file,
    // resolving the "invalid byte" error.
    exe.addCSourceFile(.{
        .file = .{ .path = "main.c" },
        .flags = &.{}, // You could add C flags like "-std=c11" here if needed.
    });

    // --- Link Libraries ---
    // We still need to tell Zig to link against the standard C library
    // for functions like printf, fopen, etc.
    exe.linkSystemLibrary("c");

    // --- Install Step ---
    // This tells the build system to install the compiled executable into the
    // `zig-out/bin/` directory.
    b.installArtifact(exe);

    // --- Create a "Run" Step ---
    // This creates the convenient `zig build run` command.
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    const run_step = b.step("run", "Run the application");
    run_step.dependOn(&run_cmd.step);
}
