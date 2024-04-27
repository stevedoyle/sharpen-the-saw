from typing import List


class Solution:
    def fizzBuzz(self, n: int) -> List[str]:
        answer = []
        for i in range(1, n + 1):
            if i % 3 == 0 and i % 5 == 0:
                answer.append("FizzBuzz")
            elif i % 3 == 0:
                answer.append("Fizz")
            elif i % 5 == 0:
                answer.append("Buzz")
            else:
                answer.append(f"{i}")
        return answer


import unittest


class TestFizzbBuzz(unittest.TestCase):
    def testFizzBuzz1(self):
        want = ["1", "2", "Fizz"]
        sln = Solution()
        self.assertEqual(want, sln.fizzBuzz(3))


if __name__ == "__main__":
    unittest.main()
