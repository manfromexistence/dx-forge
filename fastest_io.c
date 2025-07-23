#include <stdio.h>
#include <string.h>
#include <time.h>
#ifdef _WIN32
#include <windows.h>
#define THREAD_T HANDLE
#define THREAD_CREATE(thread, attr, func, arg) thread = CreateThread(NULL, 0, func, arg, 0, NULL)
#define THREAD_JOIN(thread, status) WaitForSingleObject(thread, INFINITE)
#define close CloseHandle
#define open(path, flags) CreateFileA(path, GENERIC_WRITE, 0, NULL, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, NULL)
#define write(handle, data, len) { DWORD written; WriteFile(handle, data, len, &written, NULL); }
#define mkdir(path, mode) CreateDirectoryA(path, NULL)
#else
#include <fcntl.h>
#include <unistd.h>
#include <sys/stat.h>
#include <pthread.h>
#define THREAD_T pthread_t
#define THREAD_CREATE(thread, attr, func, arg) pthread_create(&thread, attr, func, arg)
#define THREAD_JOIN(thread, status) pthread_join(thread, status)
#define mkdir(path, mode) mkdir(path, mode)
#endif

struct ThreadData {
    const char** paths;
    const char* data;
    size_t dataLen;
    int start;
    int count;
};

#ifdef _WIN32
DWORD WINAPI writeFileBatch(void* arg) {
#else
void* writeFileBatch(void* arg) {
#endif
    struct ThreadData* td = (struct ThreadData*)arg;
    for (int i = td->start; i < td->start + td->count; i++) {
        #ifdef _WIN32
        HANDLE file = open(td->paths[i], 0);
        #else
        int file = open(td->paths[i], O_WRONLY | O_CREAT | O_TRUNC, 0644);
        #endif
        if (file != -1) {
            write(file, td->data, td->dataLen);
            close(file);
        }
    }
    return NULL;
}

void createAndUpdateFiles(const char** paths, const char* data, size_t dataLen, int n) {
    #define THREAD_COUNT 8
    THREAD_T threads[THREAD_COUNT];
    struct ThreadData tds[THREAD_COUNT];
    int filesPerThread = n / THREAD_COUNT + (n % THREAD_COUNT > 0);
    for (int i = 0; i < THREAD_COUNT && i * filesPerThread < n; i++) {
        int count = filesPerThread;
        if (i * filesPerThread + count > n) count = n - i * filesPerThread;
        tds[i] = (struct ThreadData){paths, data, dataLen, i * filesPerThread, count};
        THREAD_CREATE(threads[i], NULL, writeFileBatch, &tds[i]);
    }
    for (int i = 0; i < THREAD_COUNT && i * filesPerThread < n; i++) {
        THREAD_JOIN(threads[i], NULL);
    }
}

int main() {
    struct timespec start, end;
    #ifdef _WIN32
    QueryPerformanceFrequency((LARGE_INTEGER*)&start.tv_sec);
    QueryPerformanceCounter((LARGE_INTEGER*)&start);
    mkdir("c_modules", 0755); // Create c_modules in current directory
    #else
    clock_gettime(CLOCK_MONOTONIC, &start);
    mkdir("c_modules", 0755); // Create c_modules in current directory
    #endif
    const char* data = "Hello, manfromexistence";
    size_t dataLen = strlen(data);
    const char* paths[100];
    char pathBuffers[100][64];
    for (int i = 0; i < 100; i++) {
        #ifdef _WIN32
        snprintf(pathBuffers[i], 64, "c_modules\\file_%d.txt", i);
        #else
        snprintf(pathBuffers[i], 64, "c_modules/file_%d.txt", i);
        #endif
        paths[i] = pathBuffers[i];
    }
    createAndUpdateFiles(paths, data, dataLen, 100);
    #ifdef _WIN32
    QueryPerformanceCounter((LARGE_INTEGER*)&end);
    double time_ms = ((double)(end.tv_sec - start.tv_sec) * 1000.0) / start.tv_sec;
    #else
    clock_gettime(CLOCK_MONOTONIC, &end);
    double time_ms = (end.tv_sec - start.tv_sec) * 1000.0 + (end.tv_nsec - start.tv_nsec) / 1e6;
    #endif
    printf("Time taken: %.3f ms\n", time_ms);
    return 0;
}