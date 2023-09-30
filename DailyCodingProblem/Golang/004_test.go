package dailycodingproblem

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFindMissingVal(t *testing.T) {
	input := []int{3, 4, -1, 1}
	want := 2
	assert.Equal(t, want, findMissingInt(input))
	assert.Equal(t, want, findMissingIntNaive(input))
	assert.Equal(t, want, findMissingIntTracker(input))
}

func TestFindMissingIntNoGap(t *testing.T) {
	input := []int{1, 2, 3, 4, 5, 6}
	want := 7
	assert.Equal(t, want, findMissingInt(input))
	assert.Equal(t, want, findMissingIntNaive(input))
	assert.Equal(t, want, findMissingIntTracker(input))
}

func FindMissingIntEmptyInput(t *testing.T) {
	input := []int{}
	want := 1
	assert.Equal(t, want, findMissingInt(input))
	assert.Equal(t, want, findMissingIntNaive(input))
	assert.Equal(t, want, findMissingIntTracker(input))
}
