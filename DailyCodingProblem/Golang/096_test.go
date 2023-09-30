package dailycodingproblem

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test096Permutations(t *testing.T) {

	want := [][]int{{1, 2, 3}, {1, 3, 2}, {2, 1, 3}, {2, 3, 1}, {3, 1, 2}, {3, 2, 1}}
	assert.ElementsMatch(t, want, permutations([]int{1, 2, 3}))
}
