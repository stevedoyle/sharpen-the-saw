package dailycodingproblem

import "math"

/*
Given two singly linked lists that intersect at some point, find the
intersecting node. The lists are non-cyclical.

For example, given A = 3 -> 7 -> 8 -> 10 and B = 99 -> 1 -> 8 -> 10, return the
node with value 8.

In this example, assume nodes with the same value are the exact same node
objects.

Do this in O(M + N) time (where M and N are the lengths of the lists) and
constant space.
*/

func listIntersection(a, b []int) int {
	minLen := int(math.Min(float64(len(a)), float64(len(b))))
	var asl []int
	var bsl []int
	if len(a) < len(b) {
		asl = a[:]
		bsl = b[len(b)-minLen:]
	} else {
		asl = a[len(a)-minLen:]
		bsl = b[:]
	}

	for i := 0; i < minLen; i++ {
		if asl[i] == bsl[i] {
			return asl[i]
		}
	}
	return 0
}
