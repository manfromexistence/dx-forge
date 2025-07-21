#include <stdio.h>    // For file operations (fopen, fprintf, fclose) and standard I/O (printf)
#include <stdlib.h>   // For general utilities (exit)
#include <string.h>   // For string manipulation (sprintf)
#include <sys/stat.h> // For mkdir
#include <sys/types.h> // For mkdir
#include <time.h>     // For timing the operation (clock_gettime)

// The main function is the entry point of our program.
int main() {
    // Define the directory and the content for the files.
    const char* dir_name = "c_modules_compiled_by_zig";
    const int file_count = 100;
    const char* content = "Hello, manfromexistence! (from C code compiled with Zig)";

    // Create the directory. On Linux, mkdir won't throw an error if the directory
    // already exists, it will just fail silently in that case, which is fine for us.
    // 0777 provides read, write, and execute permissions for everyone.
    mkdir(dir_name, 0777);

    // Let's start the clock! We use clock_gettime for high-precision wall-clock time,
    // which is better for I/O-bound tasks than clock().
    struct timespec start, end;
    clock_gettime(CLOCK_MONOTONIC, &start);

    // This is the main loop where the file creation happens.
    for (int i = 0; i < file_count; i++) {
        // Create a buffer to hold the full filepath string.
        char filepath[256];
        // Format the string into the buffer (e.g., "c_modules/file_0.txt").
        snprintf(filepath, sizeof(filepath), "%s/file_%d.txt", dir_name, i);

        // Open the file in "write" mode ("w"). This will create the file if it doesn't exist
        // or overwrite it if it does.
        FILE* file = fopen(filepath, "w");
        if (file == NULL) {
            // If fopen fails, print an error and exit.
            perror("Error creating file");
            return 1; // Indicate an error occurred
        }

        // Write the content to the file.
        fprintf(file, "%s", content);

        // Close the file to save changes and release the file handle.
        fclose(file);
    }

    // Stop the clock!
    clock_gettime(CLOCK_MONOTONIC, &end);

    // Calculate the elapsed time.
    // The result is in seconds and nanoseconds, so we convert it all to milliseconds.
    double elapsed_s = (end.tv_sec - start.tv_sec);
    double elapsed_ns = (end.tv_nsec - start.tv_nsec);
    double elapsed_ms = (elapsed_s * 1000.0) + (elapsed_ns / 1000000.0);

    // Print the results to the console.
    printf("\nSuccessfully created %d files in the '%s' directory.\n", file_count, dir_name);
    printf("Total time taken: %.4fms\n\n", elapsed_ms);

    return 0; // Indicate successful execution
}
