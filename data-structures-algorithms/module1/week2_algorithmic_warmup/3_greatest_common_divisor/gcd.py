import unittest


def gcd_naive(a, b):
    """Return the greatest common divisor of two integers"""
    current_gcd = 1
    for d in range(2, min(a, b) + 1):
        if a % d == 0 and b % d == 0:
            if d > current_gcd:
                current_gcd = d

    return current_gcd


def gcd(a, b):
    """Return the greatest common divisor of two integers"""
    if b == 0:
        return a
    a_prime = a % b
    return gcd(b, a_prime)


# Add a test for this problem using unittest
class TestGCD(unittest.TestCase):
    """Test for gcd function"""

    def test_gcd(self):
        """Test for gcd function"""
        self.assertEqual(gcd(18, 35), 1)
        self.assertEqual(gcd(375, 234), 3)
        self.assertEqual(gcd(28851538, 1183019), 17657)


if __name__ == "__main__":
    a, b = map(int, input().split())
    print(gcd(a, b))
