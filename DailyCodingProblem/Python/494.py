# This problem was asked by Facebook.
#
# Given a circular array, compute its maximum subarray sum in O(n) time.
# A subarray can be empty, and in this case the sum is 0.
#
# For example, given [8, -1, 3, 4], return 15 as we choose the numbers
# 3, 4, and 8 where the 8 is obtained from wrapping around.
#
# Given [-4, 5, 1, 0], return 6 as we choose the numbers 5 and 1.


def kadane(arr):
    current_sum = best_sum = 0
    for x in arr:
        current_sum = max(x, current_sum + x)
        best_sum = max(best_sum, current_sum)
    return best_sum


def max_subarray_sum(arr):
    max_wrap = sum(arr) + kadane([-x for x in arr])
    return max(kadane(arr), max_wrap)


def circularSubarraySum(arr):
    totalSum = 0
    currMaxSum = 0
    currMinSum = 0
    maxSum = arr[0]
    minSum = arr[0]

    for i in range(len(arr)):
        # Kadane's to find maximum sum subarray
        currMaxSum = max(currMaxSum + arr[i], arr[i])
        maxSum = max(maxSum, currMaxSum)

        # Kadane's to find minimum sum subarray
        currMinSum = min(currMinSum + arr[i], arr[i])
        minSum = min(minSum, currMinSum)

        # Sum of all the elements of input array
        totalSum += arr[i]

    normalSum = maxSum
    circularSum = totalSum - minSum

    # If the minimum subarray is equal to total Sum
    # then we just need to return normalSum
    if minSum == totalSum:
        return normalSum

    return max(normalSum, circularSum)


class TestMaxSubarraySum:
    def test_example1(self):
        assert max_subarray_sum([8, -1, 3, 4]) == 15
        assert circularSubarraySum([8, -1, 3, 4]) == 15

    def test_example2(self):
        assert max_subarray_sum([-4, 5, 1, 0]) == 6
        assert circularSubarraySum([-4, 5, 1, 0]) == 6

    def test_example3(self):
        assert max_subarray_sum([8, -8, 9, -9, 10, -11, 12]) == 22
        assert circularSubarraySum([8, -8, 9, -9, 10, -11, 12]) == 22

    def test_kadane(self):
        assert kadane([8, -1, 3, 4]) == 14
