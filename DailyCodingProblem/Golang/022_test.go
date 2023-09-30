package dailycodingproblem

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"golang.org/x/exp/slices"
)

func Test022ReconstructEx1(t *testing.T) {
	dictionary := []string{"quick", "brown", "the", "fox"}
	input := "thequickbrownfox"
	want := []string{"the", "quick", "brown", "fox"}
	assert.Equal(t, want, reconstruct(dictionary, input))
}

func Test022ReconstructEx2(t *testing.T) {
	dictionary := []string{"bed", "bath", "bedbath", "and", "beyond"}
	input := "bedbathandbeyond"
	want1 := []string{"bed", "bath", "and", "beyond"}
	want2 := []string{"bedbath", "and", "beyond"}
	output := reconstruct(dictionary, input)
	assert.True(t, slices.Equal(want1, output) || slices.Equal(want2, output))
}

func Test022NoValidReconstruction(t *testing.T) {
	dictionary := []string{"quick", "brown", "the", "fox"}
	input := "thequickbrownsquirrel"
	assert.Nil(t, reconstruct(dictionary, input))
}
