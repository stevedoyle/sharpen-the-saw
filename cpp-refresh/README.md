# C++ Refresh Capstones

These are the capstone projects for the Modern C++ Roadmap, which runs from C++98 to C++26 in four phases. Each capstone is a standalone CMake project that builds with Ninja.

| # | Phase | Project | Standard |
|---|-------|---------|----------|
| 01 | C++11 foundations | [Thread-safe producer-consumer queue](01-producer-consumer-queue/) | C++11 |
| 02 | C++14 / C++17 | Config-file parser (`string_view`, `optional`, `variant`) | C++17 |
| 03 | C++20 | Lazy generator pipeline (coroutines, ranges, concepts) | C++20 |
| 04 | C++23 / C++26 | `std::expected` error layer and a reflection-based serialiser | C++26 |

## Build a project

Each project has its own `CMakePresets.json`:

```bash
cd 01-producer-consumer-queue
cmake --preset debug        # configure into build/debug with Ninja
cmake --build --preset debug
ctest --preset debug
```

Requirements: CMake 3.25 or later, Ninja, and a C++ compiler for the project's standard.
