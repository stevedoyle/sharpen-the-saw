import unittest


def lcm_naive(a, b):
    """Return the least common multiple of two integers"""
    for l in range(1, a * b + 1):
        if l % a == 0 and l % b == 0:
            return l

    assert False


def gcd(a, b):
    """Return the greatest common divisor of two integers"""
    if b == 0:
        return a
    a_prime = a % b
    return gcd(b, a_prime)


def lcm(a, b):
    """Return the least common multiple of two integers"""
    return a * b // gcd(a, b)


class TestLCM(unittest.TestCase):
    """Test for lcm function"""

    def test_lcm(self):
        """Test for lcm function"""
        self.assertEqual(lcm(6, 8), 24)
        self.assertEqual(lcm(761457, 614573), 467970912861)


if __name__ == "__main__":
    a, b = map(int, input().split())
    print(lcm(a, b))
