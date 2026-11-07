// Example using C++11 nullptr

#include <cstddef>
#include <iostream>

template<class T>
constexpr T clone(const T& t)
{
    return t;
}

void g(int* x)
{
    if (x != nullptr) {
        std::cout << "Function g called with a pointer to " << *x << std::endl;
        return;
    }
    std::cout << "Function g called with null" << std::endl;
}

int main()
{
    g(nullptr);        // Fine
    g(NULL);           // Fine
    g(0);              // Fine

    g(clone(nullptr)); // Fine
//  g(clone(NULL));    // ERROR: non-literal zero cannot be a null pointer constant
//  g(clone(0));       // ERROR: non-literal zero cannot be a null pointer constant

    auto x = 5;
    g(&x);
}
