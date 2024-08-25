import unittest


def fibonacci_huge_naive(n, m):
    if n <= 1:
        return n

    previous = 0
    current = 1

    for _ in range(n - 1):
        previous, current = current, previous + current

    return current % m


def pisano_period(m):
    """Return the Pisano period for a given m"""
    previous, current = 0, 1
    for i in range(0, m * m):
        previous, current = current, (previous + current) % m
        # A Pisano period starts with 01
        if previous == 0 and current == 1:
            return i + 1
    return -1


def fibonacci_huge(n, m):
    """Return the n-th Fibonacci number modulo m using the Pisano period"""

    period = pisano_period(m)
    n = n % period

    if n <= 1:
        return n

    previous = 0
    current = 1

    for _ in range(n - 1):
        previous, current = current, (previous + current) % m

    return current


class TestFibonacciHuge(unittest.TestCase):
    """Test for fibonacci_huge function"""

    def test_fibonacci_huge(self):
        """Test for fibonacci_huge function"""
        self.assertEqual(fibonacci_huge(1, 239), 1)
        self.assertEqual(fibonacci_huge(115, 1000), 885)
        self.assertEqual(fibonacci_huge(2816213588, 239), 151)


if __name__ == "__main__":
    n, m = map(int, input().split())
    print(fibonacci_huge(n, m))
