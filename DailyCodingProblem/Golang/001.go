package dailycodingproblem

/*
Difficulty: Easy

## The Problem

This problem was recently asked by Google.

Given a list of numbers and a number k, return whether any two numbers from the
list add up to k.

For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.

Bonus: Can you do this in one pass?
*/

func checkForSum(data []int, k int) bool {
	prev := make(map[int]int)

	for _, val := range data {
		b := k - val
		_, ok := prev[b]
		if ok {
			return true
		}
		prev[val] = 1
	}

	return false
}
