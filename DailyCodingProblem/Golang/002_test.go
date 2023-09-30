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

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestProduct1(t *testing.T) {
	input := []int{1, 2, 3, 4, 5}
	want := []int{120, 60, 40, 30, 24}
	assert.Equal(t, want, product(input))
	assert.Equal(t, want, product_with_division(input))
}

func TestProduct2(t *testing.T) {
	input := []int{3, 2, 1}
	want := []int{2, 3, 6}
	assert.Equal(t, want, product(input))
	assert.Equal(t, want, product_with_division(input))
}

func TestProductEmptyList(t *testing.T) {
	input := []int{}
	want := []int{}
	assert.Equal(t, want, product(input))
	assert.Equal(t, want, product_with_division(input))
}
