/**
 * @file main.c
 * @brief Creates 1000 files using OS-native APIs for maximum speed,
 * measuring the execution time in milliseconds.
 * @author manfromexistence's Assistant (Gemini)
 *
 * Description:
 * This program is hyper-optimized for cross-platform file creation speed.
 * It uses conditional compilation to select the fastest low-level I/O API
 * available on the host operating system (POSIX for Linux/macOS, Win32 for Windows).
 * This minimizes overhead from the C standard library's buffering.
 *
 * 1. It creates a directory named "c_modules".
 * 2. It measures the total time taken in milliseconds.
 * 3. It uses multi-threading to saturate the disk's I/O capabilities.
 *
 * NOTE ON SPEED: This code is designed to be CPU-efficient to push the
 * true bottleneck—the storage drive's I/O speed—to its absolute limit.
 *
 * How to Compile:
 * Link against the POSIX threads (pthreads) library.
 * gcc main.c -o file_creator -pthread -O3
 *
 * How to Run:
 * ./file_creator
 *
 * The -O3 flag is the highest level of optimization.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <unistd.h>
#include <time.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>

// --- Platform-Specific Includes ---
#ifdef _WIN32
#include <windows.h> // For Win32 API (CreateFile, etc.)
#else
#include <fcntl.h>   // For POSIX API (open, etc.)
#endif

// --- Configuration ---
#define NUM_FILES 1000
#define TARGET_DIRECTORY "c_modules"
#define FILENAME_BUFFER_SIZE 64
#define CONTENT "Hello, manfromexistence"
#define CONTENT_LENGTH (sizeof(CONTENT) - 1) // Don't include null terminator

/**
 * @struct ThreadData
 * @brief A structure to pass data to each worker thread.
 */
typedef struct {
    int thread_id;
    int start_index;
    int end_index;
} ThreadData;

/**
 * @brief The function executed by each worker thread, using low-level I/O.
 *
 * This function creates a subset of the files using the fastest API
 * for the target OS.
 * @param arg A void pointer to a ThreadData struct.
 * @return Always returns NULL.
 */
void* create_files_task(void* arg) {
    ThreadData* data = (ThreadData*)arg;
    char filename[FILENAME_BUFFER_SIZE];

    for (int i = data->start_index; i < data->end_index; ++i) {
        snprintf(filename, FILENAME_BUFFER_SIZE, "%s/file_%d.txt", TARGET_DIRECTORY, i);

#ifdef _WIN32
        // --- Windows-Specific High-Performance Code ---
        HANDLE file_handle = CreateFileA(
            filename,
            GENERIC_WRITE,
            0,
            NULL,
            CREATE_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            NULL);

        if (file_handle == INVALID_HANDLE_VALUE) {
            // fprintf(stderr, "Error opening file %s on thread %d\n", filename, data->thread_id);
            continue;
        }

        DWORD bytes_written;
        WriteFile(
            file_handle,
            CONTENT,
            CONTENT_LENGTH,
            &bytes_written,
            NULL);

        CloseHandle(file_handle);
#else
        // --- POSIX-Specific High-Performance Code (Linux, macOS, etc.) ---
        // 0666 gives read/write permissions to user, group, and others.
        int fd = open(filename, O_WRONLY | O_CREAT | O_TRUNC, 0666);

        if (fd == -1) {
            // perror("Error opening file");
            continue;
        }

        write(fd, CONTENT, CONTENT_LENGTH);

        close(fd);
#endif
    }

    pthread_exit(NULL);
}

/**
 * @brief The main entry point of the program.
 */
int main() {
    // --- 0. Create the target directory ---
    #ifdef _WIN32
        CreateDirectoryA(TARGET_DIRECTORY, NULL);
    #else
        mkdir(TARGET_DIRECTORY, 0777);
    #endif

    // --- 1. Determine the optimal number of threads ---
    long num_threads = sysconf(_SC_NPROCESSORS_ONLN);
    if (num_threads < 1) {
        num_threads = 4;
    }

    printf("System has %ld processor cores. Using %ld threads to create %d files in './%s'.\n",
           num_threads, num_threads, NUM_FILES, TARGET_DIRECTORY);

    pthread_t threads[num_threads];
    ThreadData thread_data[num_threads];

    // --- 2. Divide the work among threads ---
    int files_per_thread = NUM_FILES / num_threads;
    int remainder_files = NUM_FILES % num_threads;
    int current_start_index = 0;

    // --- 3. Start Timer and Launch Threads ---
    struct timespec start_time, end_time;
    clock_gettime(CLOCK_MONOTONIC, &start_time);

    printf("Starting file creation...\n");
    for (int i = 0; i < num_threads; ++i) {
        thread_data[i].thread_id = i;
        thread_data[i].start_index = current_start_index;
        int num_files_for_this_thread = files_per_thread + (i < remainder_files ? 1 : 0);
        thread_data[i].end_index = current_start_index + num_files_for_this_thread;
        current_start_index = thread_data[i].end_index;

        int result = pthread_create(&threads[i], NULL, create_files_task, (void*)&thread_data[i]);
        if (result) {
            fprintf(stderr, "Error: Failed to create thread %d. Code: %d\n", i, result);
            exit(EXIT_FAILURE);
        }
    }

    // --- 4. Wait for all threads to complete ---
    for (int i = 0; i < num_threads; ++i) {
        pthread_join(threads[i], NULL);
    }

    // --- 5. Stop Timer and Report Results ---
    clock_gettime(CLOCK_MONOTONIC, &end_time);

    // Calculate elapsed time in milliseconds
    double elapsed_time_ms = (end_time.tv_sec - start_time.tv_sec) * 1000.0 +
                             (end_time.tv_nsec - start_time.tv_nsec) / 1e6;

    printf("\nTask complete! All %d files created successfully.\n", NUM_FILES);
    printf("Total execution time: %.2f ms.\n", elapsed_time_ms);

    return 0;
}
