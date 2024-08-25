import unittest


def pisano_period(m):
    """Return the Pisano period for a given m"""
    previous, current = 0, 1
    for i in range(0, m * m):
        previous, current = current, (previous + current) % m
        # A Pisano period starts with 01
        if previous == 0 and current == 1:
            return i + 1
    return -1


def fibonacci_sum(n):

    # The sum of the first n Fibonacci numbers is the (n + 2)-th Fibonacci number minus 1
    period = pisano_period(10)
    n = (n + 2) % period

    previous = 0
    current = 1

    for _ in range(n - 1):
        previous, current = current, (previous + current) % 10

    return (current - 1) % 10


class TestFibonacciSum(unittest.TestCase):
    """Test for fibonacci_sum function"""

    def test_fibonacci_sum(self):
        """Test for fibonacci_sum function"""
        self.assertEqual(fibonacci_sum(3), 4)
        self.assertEqual(fibonacci_sum(100), 5)
        self.assertEqual(fibonacci_sum(239), 0)


def fibonacci_sum_naive(n):
    """Return the last digit of the sum of the first n Fibonacci numbers"""
    if n <= 1:
        return n

    previous = 0
    current = 1
    sum = 1

    for _ in range(n - 1):
        previous, current = current, previous + current
        sum += current

    return sum % 10


if __name__ == "__main__":
    n = int(input())
    print(fibonacci_sum(n))
