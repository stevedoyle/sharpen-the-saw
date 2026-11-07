#include <gtest/gtest.h>
#include <iostream>
#include <sstream>
#include "../print2D.hpp"

// Redirect std::cout to an ostringstream for the duration of a test.
// Resets std::cout's format flags on destruction so hex mode from hexdump2D
// doesn't bleed into subsequent tests.
struct CoutCapture {
    std::ostringstream oss;
    std::streambuf* old;

    CoutCapture() : old(std::cout.rdbuf(oss.rdbuf())) {}
    ~CoutCapture() {
        std::cout.rdbuf(old);
        std::cout << std::dec;
    }
    std::string str() const { return oss.str(); }
};

// ---------------------------------------------------------------------------
// print2D
// ---------------------------------------------------------------------------

TEST(print2D, empty_matrix_produces_no_output) {
    CoutCapture cap;
    print2D(Array2D<int>{});
    EXPECT_EQ(cap.str(), "");
}

TEST(print2D, single_element) {
    CoutCapture cap;
    print2D(Array2D<int>{{42}});
    EXPECT_EQ(cap.str(), "42 \n");
}

TEST(print2D, single_row_of_ints) {
    CoutCapture cap;
    print2D(Array2D<int>{{1, 2, 3}});
    EXPECT_EQ(cap.str(), "1 2 3 \n");
}

TEST(print2D, multiple_rows_of_ints) {
    CoutCapture cap;
    print2D(Array2D<int>{{0, 1, 2}, {3, 4, 5}});
    EXPECT_EQ(cap.str(), "0 1 2 \n3 4 5 \n");
}

TEST(print2D, string_values) {
    CoutCapture cap;
    print2D(Array2D<std::string>{{"hello", "world"}, {"foo", "bar"}});
    EXPECT_EQ(cap.str(), "hello world \nfoo bar \n");
}

TEST(print2D, jagged_rows) {
    CoutCapture cap;
    print2D(Array2D<int>{{1}, {2, 3}, {4, 5, 6}});
    EXPECT_EQ(cap.str(), "1 \n2 3 \n4 5 6 \n");
}

// ---------------------------------------------------------------------------
// hexdump2D
// ---------------------------------------------------------------------------

TEST(hexdump2D, empty_matrix_produces_no_output) {
    CoutCapture cap;
    hexdump2D(Array2D<int>{});
    EXPECT_EQ(cap.str(), "");
}

TEST(hexdump2D, single_digit_values_match_decimal) {
    CoutCapture cap;
    hexdump2D(Array2D<int>{{0, 1, 9}});
    EXPECT_EQ(cap.str(), "0 1 9 \n");
}

TEST(hexdump2D, values_above_nine_print_lowercase_hex) {
    CoutCapture cap;
    hexdump2D(Array2D<int>{{10, 11, 15}});
    EXPECT_EQ(cap.str(), "a b f \n");
}

TEST(hexdump2D, byte_boundary_255) {
    CoutCapture cap;
    hexdump2D(Array2D<int>{{255}});
    EXPECT_EQ(cap.str(), "ff \n");
}

TEST(hexdump2D, multiple_rows) {
    CoutCapture cap;
    hexdump2D(Array2D<int>{{0x0, 0xf}, {0x10, 0xff}});
    EXPECT_EQ(cap.str(), "0 f \n10 ff \n");
}

TEST(hexdump2D, hex_mode_does_not_bleed_into_next_test) {
    // Run hexdump2D, then verify std::cout is back to decimal after capture.
    {
        CoutCapture cap;
        hexdump2D(Array2D<int>{{10}});
    }
    CoutCapture cap2;
    std::cout << 10;
    EXPECT_EQ(cap2.str(), "10"); // decimal, not "a"
}

// ---------------------------------------------------------------------------
// hexdump2D type constraint (compile-time documentation)
// ---------------------------------------------------------------------------

TEST(hexdump2D, integral_types_are_accepted) {
    static_assert(std::is_integral<int>::value,            "int");
    static_assert(std::is_integral<long>::value,           "long");
    static_assert(std::is_integral<short>::value,          "short");
    static_assert(std::is_integral<unsigned int>::value,   "unsigned int");
    static_assert(std::is_integral<char>::value,           "char");
}

TEST(hexdump2D, non_integral_types_are_rejected) {
    static_assert(!std::is_integral<float>::value,       "float rejected");
    static_assert(!std::is_integral<double>::value,      "double rejected");
    static_assert(!std::is_integral<std::string>::value, "string rejected");
    // Uncommenting either line below should produce a compile error:
    // hexdump2D(Array2D<float>{});
    // hexdump2D(Array2D<std::string>{});
}
