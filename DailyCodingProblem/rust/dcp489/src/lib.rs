//This problem was asked by Google.
//
//Given an array of elements, return the length of the longest subarray where all its elements are
//distinct.
//
//For example, given the array [5, 1, 3, 5, 2, 3, 4, 1], return 5 as the longest subarray of
//distinct elements is [5, 2, 3, 4, 1].

pub fn longest_subarray(arr: &[i32]) -> Vec<i32> {
    let mut longest = Vec::new();
    let mut current = Vec::new();

    for &num in arr {
        if current.contains(&num) {
            if current.len() > longest.len() {
                longest = current.clone();
            }
            current.clear();
        }
        current.push(num);
    }
    if current.len() > longest.len() {
        longest = current.clone();
    }
    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_subarray() {
        let arr = [5, 1, 3, 5, 2, 3, 4, 1];
        let result = longest_subarray(&arr);
        assert_eq!(result, vec![5, 2, 3, 4, 1]);
    }
}
