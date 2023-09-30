/*
Difficulty: Medium

This problem was asked by Uber.

Given an array of integers, return a new array such that each element at index i
of the new array is the product of all the numbers in the original array except
the one at i.

For example, if our input was [1, 2, 3, 4, 5], the expected output would be
[120, 60, 40, 30, 24]. If our input was [3, 2, 1], the expected output would be
[2, 3, 6].

Follow-up: what if you can't use division?
*/

package dailycodingproblem

func product_with_division(data []int) []int {
	n := len(data)
	var result = make([]int, n)

	prod := 1
	for _, val := range data {
		prod *= val
	}
	for i, val := range data {
		result[i] = prod / val
	}
	return result
}

func product(data []int) []int {
	n := len(data)
	var result = make([]int, n)

	for i := range data {
		prod := 1
		for _, val := range data[:i] {
			prod *= val
		}
		for _, val := range data[i+1:] {
			prod *= val
		}
		result[i] = prod
	}
	return result
}
