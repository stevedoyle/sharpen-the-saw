package dailycodingproblem

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test102FindSummers(t *testing.T) {

	assert.Equal(t, []int{2, 3, 4}, findSummers(9, []int{1, 2, 3, 4, 5}))
	assert.Equal(t, []int{2, 3}, findSummers(5, []int{1, 2, 3, 4, 5}))
}
