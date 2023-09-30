package dailycodingproblem

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test023CountSteps(t *testing.T) {
	board := [][]bool{
		{false, false, false, false},
		{true, true, false, true},
		{false, false, false, false},
		{false, false, false, false},
	}
	start := Coordinate{3, 0}
	end := Coordinate{0, 0}
	want := 7
	assert.Equal(t, want, countSteps(board, start, end))
}
