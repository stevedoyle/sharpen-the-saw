cmake_minimum_required(VERSION 3.25)

project(cpp11_features
  VERSION 0.1.0
  DESCRIPTION "C++11 language feature demos: auto, nullptr, range-for"
  LANGUAGES CXX)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

function(add_demo target src)
    add_executable(${target} ${src})
    set_target_properties(${target} PROPERTIES
      CXX_STANDARD 11
      CXX_STANDARD_REQUIRED ON
      CXX_EXTENSIONS OFF)
    target_compile_options(${target} PRIVATE
      -Wall -Wextra -Wpedantic)
endfunction()

add_demo(auto-demo    auto_demo.cpp)
add_demo(nullptr-demo nullptr_demo.cpp)
add_demo(range-demo   range_for_demo.cpp)
