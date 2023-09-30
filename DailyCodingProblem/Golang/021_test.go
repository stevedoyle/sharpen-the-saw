package dailycodingproblem

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test021RoomBooking(t *testing.T) {
	input := []Interval{{30, 75}, {0, 50}, {60, 150}}
	want := 2
	assert.Equal(t, want, rooms_needed(input))

	input = []Interval{{30, 75}, {0, 50}, {40, 150}}
	want = 3
	assert.Equal(t, want, rooms_needed(input))

	// No overlaps
	input = []Interval{{0, 40}, {50, 70}, {75, 100}}
	want = 1
	assert.Equal(t, want, rooms_needed(input))

	input = []Interval{{0, 40}, {0, 40}}
	want = 2
	assert.Equal(t, want, rooms_needed(input))
}
