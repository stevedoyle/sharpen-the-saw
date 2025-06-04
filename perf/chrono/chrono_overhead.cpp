#include <iostream>
#include <chrono>

void steadyClockOverhead() {
    // Measure the overhead of getting a starting and ending timestamp.
    auto start = std::chrono::steady_clock::now();
    auto end = std::chrono::steady_clock::now();
    std::chrono::duration<double, std::nano> duration = end - start;
    std::cout << "Steady Clock Overhead: " << duration.count() << " nanoseconds.\n";
}

void highResClockOverhead() {
    // Measure the overhead of getting a starting and ending timestamp.
    auto start = std::chrono::high_resolution_clock::now();
    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double, std::nano> duration = end - start;
    std::cout << "High Resolution Clock Overhead: " << duration.count() << " nanoseconds.\n";
}

void tscOverhead() {
    // Measure the overhead of getting a starting and ending timestamp using TSC.
    auto start = __rdtsc(); // Read Time Stamp Counter
    auto end = __rdtsc();   // Read Time Stamp Counter again
    auto duration = end - start;
    std::cout << "TSC Overhead: " << duration << " clock cycles.\n";
}

int main() {
    steadyClockOverhead();
    highResClockOverhead();
    tscOverhead();
    return 0;
}
