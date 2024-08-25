#include <iostream>

long long get_fibonacci_huge_naive(long long n, long long m) {
    if (n <= 1)
        return n;

    long long previous = 0;
    long long current  = 1;

    for (long long i = 0; i < n - 1; ++i) {
        long long tmp_previous = previous;
        previous = current;
        current = tmp_previous + current;
    }

    return current % m;
}


long long get_pisano_period(long long m) {
    long long a = 0, b = 1, c = a + b;
    for (int i = 0; i < m * m; i++) {
        c = (a + b) % m;
        a = b;
        b = c;
        if (a == 0 && b == 1)
            return i + 1;
    }
    return m;
}

// This is the efficient algorithm to compute the nth Fibonacci number modulo m using the Pisano method.
// The Pisano period is the period with which the sequence of Fibonacci numbers taken modulo m repeats.
// The Pisano period always starts with 01 and is known to be at most m^2 - 1.
// The nth Fibonacci number modulo m is the same as the nth Fibonacci number modulo the Pisano period.
// The Pisano period is used to reduce the nth Fibonacci number modulo m to the nth Fibonacci number modulo the Pisano period.
long long get_fibonacci_huge(long long n, long long m) {
    long long period = get_pisano_period(m);
    n = n % period;

    if (n <= 1) return n;

    long long previous = 0;
    long long current = 1;

    for (long long i = 0; i < n - 1; i++) {
        long long tmp_previous = previous;
        previous = current;
        current = (tmp_previous + current) % m;
    }
    return current;
}

int main() {
    long long n, m;
    std::cin >> n >> m;
//    std::cout << get_fibonacci_huge_naive(n, m) << '\n';
    std::cout << get_fibonacci_huge(n, m) << '\n';
}
