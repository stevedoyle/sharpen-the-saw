#pragma once
#include <iostream>
#include <type_traits>
#include <vector>

template<typename T>
using Array2D = std::vector<std::vector<T>>;

template<typename T>
void print2D(const Array2D<T>& data) {
    for(auto row : data) {
        for(auto value : row) {
            std::cout << value << ' ';
        }
        std::cout << std::endl;
    }
}

template<typename T>
typename std::enable_if<std::is_integral<T>::value>::type
hexdump2D(const Array2D<T>& data) {
    for(auto row : data) {
        for(auto value : row) {
            std::cout << std::hex << value << ' ';
        }
        std::cout << std::endl;
    }
}
