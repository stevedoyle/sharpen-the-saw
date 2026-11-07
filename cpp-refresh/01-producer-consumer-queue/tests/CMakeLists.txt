include(FetchContent)

FetchContent_Declare(googletest
  URL https://github.com/google/googletest/archive/refs/tags/v1.17.0.tar.gz
  DOWNLOAD_EXTRACT_TIMESTAMP TRUE
  FIND_PACKAGE_ARGS NAMES GTest)
set(INSTALL_GTEST OFF CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

add_executable(pcq-tests blocking_queue_test.cpp)
target_link_libraries(pcq-tests PRIVATE pcq::pcq pcq_options GTest::gtest_main)
# Match the library's standard (always C++20 or later, which also covers
# GoogleTest 1.17's C++17 minimum).
set_target_properties(pcq-tests PROPERTIES
  CXX_STANDARD ${PCQ_CXX_STANDARD}
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS OFF)

include(GoogleTest)
gtest_discover_tests(pcq-tests)

# End-to-end checks for pcq-demo; see demo_pipeline_test.sh for each case.
foreach(case every-line-once seq-matches-line empty-input rejects-bad-usage)
  add_test(NAME pcq-demo.${case}
    COMMAND sh ${CMAKE_CURRENT_SOURCE_DIR}/demo_pipeline_test.sh
            $<TARGET_FILE:pcq-demo> ${case})
endforeach()
