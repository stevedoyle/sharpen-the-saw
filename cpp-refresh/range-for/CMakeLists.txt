cmake_minimum_required(VERSION 3.25)

project(range_demo
  VERSION 0.1.0
  DESCRIPTION "C++11 range-for demo"
  LANGUAGES CXX)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(range-demo main.cpp)
set_target_properties(range-demo PROPERTIES
  CXX_STANDARD 11
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS OFF)
target_compile_options(range-demo PRIVATE
  -Wall -Wextra -Wpedantic)
