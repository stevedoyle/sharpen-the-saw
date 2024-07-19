/*
This problem was asked by Twitter.

Given a string, sort it in decreasing order based on the frequency of characters. If there are
multiple possible solutions, return any of them.

For example, given the string tweet, return tteew. eettw would also be acceptable.
*/

pub fn sort(s: &str) -> String {
    let mut char_count = std::collections::HashMap::new();
    for c in s.chars() {
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
    }

    let mut char_count_vec: Vec<(char, usize)> = char_count.into_iter().collect();
    char_count_vec.sort_by(|a, b| b.1.cmp(&a.1));

    let mut result = String::new();
    for (c, count) in char_count_vec {
        for _ in 0..count {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(sort("tweet"), "eettw");
    }
}
