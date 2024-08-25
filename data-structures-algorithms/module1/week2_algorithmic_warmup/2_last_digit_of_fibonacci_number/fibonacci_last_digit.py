import unittest


def fibonacci_last_digit_naive(n):
    """Return the last digit of the n-th Fibonacci number"""
    if n <= 1:
        return n

    previous = 0
    current = 1

    for _ in range(n - 1):
        previous, current = current, previous + current

    return current % 10


def pisano_period(m):
    """Return the Pisano period for a given m"""
    previous, current = 0, 1
    for i in range(0, m * m):
        previous, current = current, (previous + current) % m
        # A Pisano period starts with 01
        if previous == 0 and current == 1:
            return i + 1
    return -1


def fibonacci_last_digit(n):
    """Return the n-th Fibonacci number modulo m using the Pisano period"""

    period = pisano_period(10)
    n = n % period

    if n <= 1:
        return n

    previous = 0
    current = 1

    for _ in range(n - 1):
        previous, current = current, (previous + current) % 10

    return current % 10


class TestFibonacciLastDigit(unittest.TestCase):
    """Test for fibonacci_last_digit function"""

    def test_fibonacci_last_digit(self):
        """Test for fibonacci_last_digit function"""
        self.assertEqual(fibonacci_last_digit(3), 2)
        self.assertEqual(fibonacci_last_digit(331), 9)
        self.assertEqual(fibonacci_last_digit(327305), 5)


if __name__ == "__main__":
    n = int(input())
    print(fibonacci_last_digit(n))
