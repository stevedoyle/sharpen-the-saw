/*
Given an even number (greater than 2), return two prime numbers whose sum will
be equal to the given number.

A solution will always exist. See Goldbach’s conjecture.

Example:

Input: 4
Output: 2 + 2 = 4

If there are more than one solution possible, return the lexicographically
smaller solution.

If [a, b] is one solution with a <= b, and [c, d] is another solution with
c <= d, then
	[a, b] < [c, d] If a < c OR a==c AND b < d.
*/

package dailycodingproblem

import (
	"github.com/fxtlabs/primes"
)

func getPrimeSum(val int) []int {
	if val == 2 {
		return []int{1, 1}
	}

	ps := primes.Sieve(val)

	for i, a := range ps {
		for _, b := range ps[i:] {
			if a+b == val {
				return []int{a, b}
			}
		}
	}
	return []int{}
}
