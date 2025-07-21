#include <iostream>   // For standard I/O (std::cout)
#include <fstream>    // For file streams (std::ofstream)
#include <string>     // For using the std::string class
#include <chrono>     // For high-precision timing
#include <filesystem> // For modern directory and path operations (C++17)

// The main function is the entry point of our program.
int main() {
    // Define the directory and the content for the files.
    const std::string dir_name = "cpp_modules";
    const int file_count = 100;
    const std::string content = "Hello, manfromexistence!";

    // Create a path object for our directory.
    std::filesystem::path dir_path(dir_name);

    // Create the directory. std::filesystem::create_directory
    // won't throw an error if the directory already exists.
    try {
        std::filesystem::create_directory(dir_path);
    } catch (const std::filesystem::filesystem_error& e) {
        // Handle potential errors during directory creation, though it's unlikely here.
        std::cerr << "Error creating directory: " << e.what() << std::endl;
        return 1; // Indicate an error
    }

    // Let's start the clock! We use the high_resolution_clock for the best precision.
    auto start = std::chrono::high_resolution_clock::now();

    // This is the main loop where the file creation happens.
    for (int i = 0; i < file_count; ++i) {
        // Construct the full filepath using the modern path object and string manipulation.
        std::filesystem::path file_path = dir_path / ("file_" + std::to_string(i) + ".txt");

        // Open the file for writing. std::ofstream creates the file if it doesn't exist.
        // The file is automatically closed when `outFile` goes out of scope at the end of the loop,
        // thanks to C++'s RAII (Resource Acquisition Is Initialization) principle.
        std::ofstream outFile(file_path);
        if (!outFile.is_open()) {
            std::cerr << "Error creating file: " << file_path << std::endl;
            continue; // Skip to the next file on error
        }

        // Write the content to the file using the stream insertion operator.
        outFile << content;
    }

    // Stop the clock!
    auto end = std::chrono::high_resolution_clock::now();

    // Calculate the elapsed time and cast it to milliseconds.
    std::chrono::duration<double, std::milli> elapsed = end - start;

    // Print the results to the console.
    std::cout << "\nSuccessfully created " << file_count << " files in the '" << dir_name << "' directory." << std::endl;
    std::cout << "Total time taken: " << elapsed.count() << "ms\n" << std::endl;

    return 0; // Indicate successful execution
}
