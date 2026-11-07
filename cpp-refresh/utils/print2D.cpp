#include "print2D.hpp"

int main() {
    Array2D<int> matrix = {
        {0, 1, 2, 3},
        {4, 5, 6, 7},
        {8, 9, 10, 11},
    };

    print2D(matrix);
    hexdump2D(matrix);

    Array2D<std::string> str2d = {
        {"abc", "def", "ghi"},
        {"jkl", "mno", "pqr"},
        {"stu", "vwx", "yz"},
    };

    print2D(str2d);

    return 0;
}
