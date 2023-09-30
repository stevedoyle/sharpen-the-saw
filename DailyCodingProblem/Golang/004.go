/*
Difficulty: Hard

## The Problem

This problem was asked by Stripe.

Given an array of integers, find the first missing positive integer in
linear time and constant space. In other words, find the lowest positive
integer that does not exist in the array. The array can contain
duplicates and negative numbers as well.

For example, the input [3, 4, -1, 1] should give 2. The input [1, 2, 0]
should give 3.

You can modify the input array in-place.
*/

package dailycodingproblem

/*
Naive approach.
Time complexity: O(n^2)
Space complexity: O(1)
*/
func findMissingIntNaive(data []int) int {
	n := len(data)

	for i := 1; i <= n; i++ {
		found := false
		for _, val := range data {
			if i == val {
				found = true
				break
			}
		}
		if !found {
			return i
		}
	}
	return n + 1
}

/*
Build a tracker to track the positive integers
Time complexity: O(n) ... assuming map operations are O(1)
Space complexity: O(n)
*/
func findMissingIntTracker(data []int) int {
	n := len(data)

	tracker := make(map[int]int, n)

	for _, val := range data {
		if val > 0 && val <= n {
			tracker[val] = 1
		}
	}

	for i := 1; i <= n; i++ {
		_, ok := tracker[i]
		if !ok {
			return i
		}
	}

	return n + 1
}

/*
Place the positive integers in the array at the index given by their
value, i.e. data[idx] = idx
Time complexity: O(n)
Space complexity: O(1)
*/
func findMissingInt(data []int) int {
	n := len(data)

	for i := 0; i < n; i++ {
		want := i + 1
		for data[i] > 0 {
			if data[i] == want {
				break
			}
			if data[i] <= 0 || data[i] > n {
				data[i] = -1
				break
			}
			j := data[i] - 1
			data[j], data[i] = data[i], data[j]
		}
	}

	// Find the missing value
	for i := 1; i <= n; i++ {
		if data[i-1] != i {
			return i
		}
	}

	// No gap found. The next value in the sequence comes after the last value
	// in the array.
	return n + 1
}
