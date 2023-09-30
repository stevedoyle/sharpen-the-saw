package dailycodingproblem

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test101GoldbachSingleAnwer(t *testing.T) {

	assert.Equal(t, []int{1, 1}, getPrimeSum(2))
	assert.Equal(t, []int{2, 2}, getPrimeSum(4))
	assert.Equal(t, []int{3, 3}, getPrimeSum(6))
	assert.Equal(t, []int{3, 5}, getPrimeSum(8))
	assert.Equal(t, []int{3, 7}, getPrimeSum(10))
	assert.Equal(t, []int{5, 7}, getPrimeSum(12))
	assert.Equal(t, []int{3, 97}, getPrimeSum(100))
}
