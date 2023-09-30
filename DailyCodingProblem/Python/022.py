"""
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
"""

import re
import unittest

def reconstruct(dictionary, input):
    """
    Approach: Use the dictionary contents to direct the matching.
    Construct a regex pattern that is a grouping of all words in the
    dictionary and use this with a non-greedy match against the input
    string. Reconstruction is achieved if the entire input string is
    consumed and forms a set of matches.
    """
    pattern = '(' + '|'.join(dictionary) + ')+?'
    m = re.findall(pattern, input)
    if not m:
        return []
    if len(''.join(m)) != len(input):
        return []
    return m

class Test022Reconstruction(unittest.TestCase):
    def testReconstruction1(self):
        dictionary = ['quick', 'brown', 'the', 'fox']
        input = 'thequickbrownfox'
        want = ['the', 'quick', 'brown', 'fox']
        self.assertEqual(want, reconstruct(dictionary, input))

    def testReconstruction2(self):
        dictionary = ['bed', 'bath', 'bedbath', 'and', 'beyond']
        input = 'bedbathandbeyond'
        mandatory = ['and', 'beyond']
        optionals = [['bed', 'bath'], ['bedbath']]
        output = reconstruct(dictionary, input)
        self.assertEqual(mandatory, output[-2:])
        self.assertTrue(optionals[0] == output[:-2] or
                        optionals[1] == output[:-2])

    def testNoValidReconstruction(self):
        dictionary = ['quick', 'brown', 'the', 'fox']
        input = 'thequickbrownsquirrel'
        want = []
        self.assertEqual(want, reconstruct(dictionary, input))

if __name__ == "__main__":
    unittest.main()