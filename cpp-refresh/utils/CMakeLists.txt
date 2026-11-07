cmake_minimum_required(VERSION 3.25)

set(TARGET print2D-demo)

project(${TARGET}
  VERSION 0.1.0
  DESCRIPTION "C++11 range-for demo"
  LANGUAGES CXX)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(${TARGET} print2D.cpp)
set_target_properties(${TARGET} PROPERTIES
  CXX_STANDARD 11
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS OFF)
target_compile_options(${TARGET} PRIVATE
  -Wall -Wextra -Wpedantic)

option(BUILD_TESTS "Build unit tests (fetches GoogleTest)" ON)
if(BUILD_TESTS)
  enable_testing()
  add_subdirectory(tests)
endif()
