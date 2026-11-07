cmake_minimum_required(VERSION 3.25)

set(TARGET nullptr-demo)

project(${TARGET}
  VERSION 0.1.0
  DESCRIPTION "auto keyword demo"
  LANGUAGES CXX)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(${TARGET} main.cpp)
set_target_properties(${TARGET} PROPERTIES
  CXX_STANDARD 11
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS OFF)
target_compile_options(${TARGET} PRIVATE
  -Wall -Wextra -Wpedantic)
