import unittest


def fibonacci_sum_squares(n):
    """Return the last digit of the sum of the squares of the first n Fibonacci numbers"""

    # The sum of the squares of the first n Fibonacci numbers is the n-th Fibonacci number times the (n + 1)-th Fibonacci number
    # The last digit of the sum of the squares of the first n Fibonacci numbers is the last digit of the n-th Fibonacci number times the last digit of the (n + 1)-th Fibonacci number

    if n <= 1:
        return n

    previous = 0
    current = 1

    # The Pisano period for 10 is 60
    for _ in range(n % 60):
        previous, current = current, (previous + current) % 10

    return (previous * current) % 10


class TestFibonacciSumSquares(unittest.TestCase):
    """Test for fibonacci_sum_squares function"""

    def test_fibonacci_sum_squares(self):
        """Test for fibonacci_sum_squares function"""
        self.assertEqual(fibonacci_sum_squares(7), 3)
        self.assertEqual(fibonacci_sum_squares(73), 1)


if __name__ == "__main__":
    n = int(input())
    print(fibonacci_sum_squares(n))
