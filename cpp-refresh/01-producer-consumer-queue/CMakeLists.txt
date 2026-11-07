cmake_minimum_required(VERSION 3.25)

project(producer_consumer_queue
  VERSION 0.1.0
  DESCRIPTION "Phase 1 capstone: thread-safe producer-consumer queue (C++11)"
  LANGUAGES CXX)

# The library and demo build as C++23 by default. C++20 is the minimum:
# pcq-demo uses std::make_unique with parenthesised aggregate init.
# Tests use the same standard.
set(PCQ_CXX_STANDARD 23 CACHE STRING "C++ standard for the library and demo")
set_property(CACHE PCQ_CXX_STANDARD PROPERTY STRINGS 20 23)
if(NOT PCQ_CXX_STANDARD MATCHES "^(20|23)$")
  message(FATAL_ERROR
    "PCQ_CXX_STANDARD must be 20 or 23 (got '${PCQ_CXX_STANDARD}')")
endif()

option(PCQ_BUILD_TESTS "Build unit tests (fetches GoogleTest)" ON)
option(PCQ_WARNINGS_AS_ERRORS "Treat compiler warnings as errors" OFF)
set(PCQ_SANITIZER "" CACHE STRING "Sanitizer to enable: '', thread, address")
set_property(CACHE PCQ_SANITIZER PROPERTY STRINGS "" thread address)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(Threads REQUIRED)

# Shared build settings: warnings + optional sanitizer.
add_library(pcq_options INTERFACE)
target_compile_options(pcq_options INTERFACE
  -Wall -Wextra -Wpedantic -Wshadow -Wconversion -Wnon-virtual-dtor
  $<$<BOOL:${PCQ_WARNINGS_AS_ERRORS}>:-Werror>)
if(PCQ_SANITIZER)
  target_compile_options(pcq_options INTERFACE
    -fsanitize=${PCQ_SANITIZER} -fno-omit-frame-pointer)
  target_link_options(pcq_options INTERFACE -fsanitize=${PCQ_SANITIZER})
endif()

# The queue itself: header-only (it is a class template).
add_library(pcq INTERFACE)
add_library(pcq::pcq ALIAS pcq)
target_include_directories(pcq INTERFACE
  $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>)
target_link_libraries(pcq INTERFACE Threads::Threads)

# Demo: a stdin -> queue -> stdout pipeline.
add_executable(pcq-demo src/main.cpp)
target_link_libraries(pcq-demo PRIVATE pcq::pcq pcq_options)
set_target_properties(pcq-demo PROPERTIES
  CXX_STANDARD ${PCQ_CXX_STANDARD}
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS OFF)

if(PCQ_BUILD_TESTS)
  enable_testing()
  add_subdirectory(tests)

  # End-to-end pipeline checks for pcq-demo, driven through the shell.
  set(PCQ_DEMO "$<TARGET_FILE:pcq-demo>")
  add_test(NAME pcq-demo.every-line-once
    COMMAND sh -c "test \"$(seq 1 5000 | '${PCQ_DEMO}' 8 | sort -n | cut -f2- | cksum)\" = \"$(seq 1 5000 | cksum)\"")
  add_test(NAME pcq-demo.seq-matches-line
    COMMAND sh -c "seq 1 1000 | '${PCQ_DEMO}' 4 | awk -F'\\t' '$1 != $2 { exit 1 }'")
  add_test(NAME pcq-demo.empty-input
    COMMAND sh -c "test -z \"$('${PCQ_DEMO}' 3 < /dev/null)\"")
  add_test(NAME pcq-demo.rejects-bad-consumer-count
    COMMAND sh -c "for a in 0 -1 4x '' 99999; do '${PCQ_DEMO}' \"$a\" < /dev/null 2>/dev/null && exit 1; done; exit 0")
endif()
