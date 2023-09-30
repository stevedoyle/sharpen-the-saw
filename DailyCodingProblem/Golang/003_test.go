package dailycodingproblem

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSerialize(t *testing.T) {
	llnode := &Node{"left.left", nil, nil}
	lnode := &Node{"left", llnode, nil}
	node := Node{"root", lnode, &Node{"right", nil, nil}}

	want := "root,left,left.left,,,,right,,,"
	assert.Equal(t, want, serialize(&node))
	tree := deserialize(serialize(&node))
	if assert.NotNil(t, tree) &&
		assert.NotNil(t, tree.left) &&
		assert.NotNil(t, tree.left.left) {
		assert.Equal(t, "left.left", tree.left.left.val)
	}
}
