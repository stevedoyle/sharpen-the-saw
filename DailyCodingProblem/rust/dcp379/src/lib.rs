// This problem was asked by Microsoft.
//
// Given a string, generate all possible subsequences of the string.
//
// For example, given the string xyz, return an array or set with the following strings:
//
//
// x
// y
// z
// xy
// xz
// yz
// xyz
// Note that zx is not a valid subsequence since it is not in the order of the given string.

pub fn subsequences(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut chars: Vec<char> = s.chars().collect();
    let mut current = String::new();
    subsequences_helper(&mut chars, &mut current, &mut result);
    result
}

fn subsequences_helper(chars: &mut Vec<char>, current: &mut String, result: &mut Vec<String>) {
    if chars.is_empty() {
        if current.len() > 0 {
            result.push(current.clone());
        }
        return;
    }

    let c = chars.remove(0);
    subsequences_helper(chars, current, result);
    current.push(c);
    subsequences_helper(chars, current, result);
    current.pop();
    chars.insert(0, c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = subsequences("xyz");
        let mut expected = vec!["x", "y", "z", "xy", "xz", "yz", "xyz"];
        expected.sort();
        result.sort();
        assert_eq!(result, expected);
    }
}
