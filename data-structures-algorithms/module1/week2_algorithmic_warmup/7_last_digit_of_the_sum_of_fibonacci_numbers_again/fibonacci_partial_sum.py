# Uses python3
import sys
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


def fibonacci_partial_sum(from_, to):
    return (fibonacci_sum(to) - fibonacci_sum(from_ - 1)) % 10


class TestFibonacciPartialSum(unittest.TestCase):
    """Test for fibonacci_partial_sum_naive function"""

    def test_fibonacci_partial_sum_naive(self):
        """Test for fibonacci_partial_sum_naive function"""
        self.assertEqual(fibonacci_partial_sum(3, 7), 1)
        self.assertEqual(fibonacci_partial_sum(10, 10), 5)
        self.assertEqual(fibonacci_partial_sum(10, 200), 2)
        self.assertEqual(fibonacci_partial_sum(0, 0), 0)


if __name__ == "__main__":
    input = sys.stdin.read()
    from_, to = map(int, input.split())
    print(fibonacci_partial_sum(from_, to))
