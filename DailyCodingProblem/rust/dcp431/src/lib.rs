/*
This problem was asked by Nest.

Create a basic sentence checker that takes in a stream of characters and determines whether they
form valid sentences. If a sentence is valid, the program should print it out.

We can consider a sentence valid if it conforms to the following rules:

The sentence must start with a capital letter, followed by a lowercase letter or a space.
All other characters must be lowercase letters, separators (,,;,:) or terminal marks (.,?,!,‽).
There must be a single space between each word.
The sentence must end with a terminal mark immediately following a word.
*/

use regex::Regex;

pub fn is_sentence(sentence: &str) -> bool {
    // regex pattern to match the sentence
    // Begin with an upper case letter: ^[A-Z]
    // Followed by a lower case letter or a space: ([a-z]+| )
    // Followed by a lower case letter or a separator or one space: ([a-z]+|[,;: ]?)*
    // End with a terminal mark immediately following a word: [a-z]+[.?!]$
    let pattern = Regex::new(r"^[A-Z]([a-z]+| )([a-z]+|[,;: ]?)*[a-z]+[.?!]$").unwrap();
    pattern.is_match(sentence)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sentence() {
        assert_eq!(is_sentence("Hello world!"), true);
        assert_eq!(is_sentence("Hello World"), false); // missing terminal mark
        assert_eq!(is_sentence("Hello World."), false); // uppercase letter in the middle
        assert_eq!(is_sentence("Hello, world!"), true);
        assert_eq!(is_sentence("A sentence."), true);
        assert_eq!(is_sentence("a sentence."), false); // missing capital letter at start
        assert_eq!(is_sentence("A  sentence."), true); // double space
        assert_eq!(
            is_sentence("A sentence with a space before the terminal mark ."),
            false
        ); // space before terminal mark
    }
}
