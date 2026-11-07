// Example of C++11 range-for

#include <iostream>
#include <vector>

using namespace std;

int main() {
    vector<int> data = {1, 2, 3, 4, 5, 6};

    for (auto i : data) {
        cout << i << ' ';
    }
    cout << endl;

    int a[] = {0, 1, 2, 3, 4, 5};
    for (int n : a) // the initializer may be an array
        std::cout << n << ' ';
    std::cout << '\n';

    return 0;
}
