cmake_minimum_required(VERSION 3.25)

project(auto_demo
  VERSION 0.1.0
  DESCRIPTION "auto keyword demo"
  LANGUAGES CXX)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(auto-demo main.cpp)
set_target_properties(auto-demo PROPERTIES
  CXX_STANDARD 11
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS OFF)
target_compile_options(auto-demo PRIVATE
  -Wall -Wextra -Wpedantic)
