package dailycodingproblem

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCheckForSum1(t *testing.T) {
	data := []int{10, 15, 3, 7}
	assert.Equal(t, true, checkForSum(data, 17))
}

func TestCheckForSum2(t *testing.T) {
	data := []int{10, 15, 3, 7}
	assert.Equal(t, false, checkForSum(data, 16))
}

func TestCheckForSumEmptyList(t *testing.T) {
	data := []int{}
	assert.Equal(t, false, checkForSum(data, 17))
}
