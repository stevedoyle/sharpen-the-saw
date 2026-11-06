cmake_minimum_required(VERSION 3.25)

project(producer_consumer_queue
  VERSION 0.1.0
  DESCRIPTION "Phase 1 capstone: thread-safe producer-consumer queue (C++11)"
  LANGUAGES CXX)

# Phase 1 is about C++11. The library and demo are pinned to C++11 so that
# reaching for a later feature (std::make_unique, generic lambdas, ...) is a
# compile error, not a silent upgrade. Tests may use a newer standard.
set(PCQ_CXX_STANDARD 11 CACHE STRING "C++ standard for the library and demo")

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
endif()
