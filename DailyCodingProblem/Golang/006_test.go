package dailycodingproblem

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test006XorLinkedList(t *testing.T) {
	assert.Equal(t, 3, car(cons(3, 4)))
	assert.Equal(t, 4, cdr(cons(3, 4)))
}
