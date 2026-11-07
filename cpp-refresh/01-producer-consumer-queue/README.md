# 01 - Thread-safe producer-consumer queue

Phase 1 capstone (C++11 foundations). Build `pcq::BlockingQueue<T>`, a bounded multi-producer/multi-consumer queue. It must work with move-only types such as `std::unique_ptr<Job>` and never copy an element.

## Layout

```text
include/blocking_queue.hpp       the queue (header-only template)
src/main.cpp                     pcq-demo: stdin -> queue -> N consumers -> stdout
tests/blocking_queue_test.cpp    GoogleTest specs
tests/demo_pipeline_test.sh      end-to-end checks for pcq-demo (run by ctest)
```

## Standards

- The library, `pcq-demo` and the tests compile as C++23 by default (`CXX_EXTENSIONS OFF`). Set `-DPCQ_CXX_STANDARD=20` to use C++20.
- C++20 is the minimum: `pcq-demo` calls `std::make_unique<Job>(seq, line)`, which relies on C++20 parenthesised aggregate initialisation. `blocking_queue.hpp` itself still uses only C++11 features.

## Presets

| Preset | Purpose |
|--------|---------|
| `debug` | Normal development build |
| `release` | Optimised build |
| `tsan` | ThreadSanitizer, use it for every concurrency test |
| `asan` | AddressSanitizer and leak checks |

```bash
cmake --preset tsan && cmake --build --preset tsan && ctest --preset tsan
```

Other options: `-DPCQ_WARNINGS_AS_ERRORS=ON`, `-DPCQ_BUILD_TESTS=OFF`, and `-DPCQ_CXX_STANDARD=20`. An existing build directory keeps its cached standard, so reconfigure with `cmake --fresh --preset <name>` after changing the default.

## Demo

```bash
seq 1 1000 | ./build/debug/pcq-demo 4 | sort -n | tail
```

`pcq-demo [consumers]` reads lines from stdin and writes each one as `<seq>\t<line>`, where `<seq>` is the 1-based input line number. `consumers` defaults to 2 and must be between 1 and 1024. Consumers run concurrently, so output order varies between runs. To restore input order:

```bash
./build/debug/pcq-demo 4 < input.txt | sort -n | cut -f2-
```

`ctest` also runs end-to-end pipeline checks on the demo (`pcq-demo.*`).

## Checklist

- [x] `push` / `pop` move items through the queue and work with `std::unique_ptr`
- [x] `push` blocks when the queue is full (bounded capacity)
- [x] `try_push` / `try_pop` never block
- [x] `close()` wakes every waiter. Consumers drain the remaining items, then `pop` returns false
- [x] Concurrency tests pass with many producers and many consumers, clean under `tsan`
- [x] `pcq-demo` pipeline works
- [ ] Stretch: `pop_for(timeout)` using `std::chrono`
