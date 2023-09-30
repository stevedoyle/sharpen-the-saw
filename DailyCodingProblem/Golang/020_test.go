package dailycodingproblem

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test020ListIntersection(t *testing.T) {
	a := []int{3, 7, 8, 10}
	b := []int{99, 1, 8, 10}
	want := 8
	assert.Equal(t, want, listIntersection(a, b))

	a = []int{3, 7, 8, 10}
	b = []int{99, 98, 97, 96}
	want = 0
	assert.Equal(t, want, listIntersection(a, b))

	a = []int{3, 7, 8, 10}
	b = []int{99, 98, 97, 96, 8, 10}
	want = 8
	assert.Equal(t, want, listIntersection(a, b))
}
