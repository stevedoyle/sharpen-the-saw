/*
This problem was asked by Robinhood.

Given an array of strings, group anagrams together.

For example, given the following array:

['eat', 'ate', 'apt', 'pat', 'tea', 'now']
Return:

[['eat', 'ate', 'tea'],
 ['apt', 'pat'],
 ['now']]
*/

pub fn group_anagrams(strs: &Vec<String>) -> Vec<Vec<String>> {
    use itertools::Itertools;
    use std::collections::HashMap;

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for s in strs {
        let key = s.chars().sorted().collect();
        map.entry(key).or_default().push(s.to_string());
    }
    map.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec!["eat", "ate", "apt", "pat", "tea", "now"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let output = vec![
            vec!["eat".to_string(), "ate".to_string(), "tea".to_string()],
            vec!["apt".to_string(), "pat".to_string()],
            vec!["now".to_string()],
        ];
        assert!(group_anagrams(&input)
            .iter()
            .all(|item| output.contains(item)));
    }
}
