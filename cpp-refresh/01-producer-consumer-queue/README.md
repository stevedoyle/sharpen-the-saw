# 01 - Thread-safe producer-consumer queue

Phase 1 capstone (C++11 foundations). Build `pcq::BlockingQueue<T>`, a bounded multi-producer/multi-consumer queue. It must work with move-only types such as `std::unique_ptr<Job>` and never copy an element.

## Layout

```text
include/blocking_queue.hpp       the queue (header-only template), the part to implement
src/main.cpp                     pcq-demo: stdin -> queue -> N consumers -> stdout
tests/blocking_queue_test.cpp    GoogleTest specs, DISABLED_ until implemented
```

## Standards

- The library and `pcq-demo` compile as strict C++11 (`CXX_EXTENSIONS OFF`). C++14 features such as `std::make_unique` cause a compile error.
- The tests compile as C++17 because GoogleTest 1.17 requires it. `pcq-demo` includes the same header, so it still has to compile as C++11.

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

Other options: `-DPCQ_WARNINGS_AS_ERRORS=ON`, `-DPCQ_BUILD_TESTS=OFF`, and `-DPCQ_CXX_STANDARD=14` (for example, to compare standards).

## Demo

```bash
seq 1 1000 | ./build/debug/pcq-demo 4 | sort -n | tail
```

## Checklist

- [ ] `push` / `pop` move items through the queue and work with `std::unique_ptr`
- [ ] `push` blocks when the queue is full (bounded capacity)
- [ ] `try_push` / `try_pop` never block
- [ ] `close()` wakes every waiter. Consumers drain the remaining items, then `pop` returns false
- [ ] Concurrency tests pass with many producers and many consumers, clean under `tsan`
- [ ] `pcq-demo` pipeline works
- [ ] Stretch: `pop_for(timeout)` using `std::chrono`
