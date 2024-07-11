// This problem was asked by Gusto.
//
// Implement the function embolden(s, lst) which takes in a string s and list of substrings lst, and
// wraps all substrings in s with an HTML bold tag <b> and </b>.
//
// If two bold tags overlap or are contiguous, they should be merged.
//
// For example, given s = abcdefg and lst = ["bc", "ef"], return the string a<b>bc</b>d<b>ef</b>g.
//
// Given s = abcdefg and lst = ["bcd", "def"], return the string a<b>bcdef</b>g, since they overlap.

pub fn embolden(s: &str, lst: Vec<&str>) -> String {
    // find all occurences of words in lst in s and record their positions
    let mut bold_regions = Vec::new();
    for word in lst.iter() {
        let matches: Vec<_> = s.match_indices(word).collect();
        for (start, _) in matches {
            bold_regions.push((start, start + word.len()));
        }
    }

    // sort bold regions by start index
    bold_regions.sort();

    // merge overlapping regions
    let mut i = 0;
    while i < bold_regions.len() - 1 {
        if bold_regions[i].1 >= bold_regions[i + 1].0 {
            bold_regions[i].1 = bold_regions[i + 1].1;
            bold_regions.remove(i + 1);
        }
        i += 1;
    }

    // construct the result string
    let mut result = String::new();

    let mut i = 0;
    for (start, end) in bold_regions {
        result.push_str(&s[i..start]);
        result.push_str("<b>");
        result.push_str(&s[start..end]);
        result.push_str("</b>");
        i = end;
    }

    result.push_str(&s[i..]);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_overlap() {
        let result = embolden("abcdefg", vec!["bc", "ef"]);
        assert_eq!(result, "a<b>bc</b>d<b>ef</b>g");
    }

    #[test]
    fn overlap() {
        let result = embolden("abcdefg", vec!["bcd", "def"]);
        assert_eq!(result, "a<b>bcdef</b>g");
    }

    #[test]
    fn multimatch() {
        let result = embolden("abcdefgbcd", vec!["bcd", "def"]);
        assert_eq!(result, "a<b>bcdef</b>g<b>bcd</b>");
    }

    #[test]
    fn single_region() {
        let result = embolden("abcdefg", vec!["bcd"]);
        assert_eq!(result, "a<b>bcd</b>efg");
    }
}
