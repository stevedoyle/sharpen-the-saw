/*
This problem was asked by Microsoft.

Given a dictionary of words and a string made up of those words (no
spaces), return the original sentence in a list. If there is more than
one possible reconstruction, return any of them. If there is no possible
reconstruction, then return null.

For example, given the set of words 'quick', 'brown', 'the', 'fox', and
the string "thequickbrownfox", you should return ['the', 'quick',
'brown', 'fox'].

Given the set of words 'bed', 'bath', 'bedbath', 'and', 'beyond', and
the string "bedbathandbeyond", return either ['bed', 'bath', 'and',
'beyond] or ['bedbath', 'and', 'beyond'].
*/

package dailycodingproblem

import (
	"regexp"
	"strings"
)

func reconstruct(dictionary []string, input string) []string {
	/*
	   Approach: Use the dictionary contents to direct the matching. Construct a
	   regex pattern that is a grouping of all words in the dictionary and use
	   this with a non-greedy match against the input string. Reconstruction is
	   achieved if the entire input string is consumed and forms a set of
	   matches.
	*/

	patternStr := "(" + strings.Join(dictionary, "|") + ")+?"
	pattern := regexp.MustCompile(patternStr)
	m := pattern.FindAllString(input, -1)
	if m != nil {
		// Check that the matches cover the entire input string
		if len(strings.Join(m, "")) != len(input) {
			return nil
		}
	}
	return m
}
