/*
This problem was asked by Facebook.

Given an array of integers in which two elements appear exactly once and all other elements appear
exactly twice, find the two elements that appear only once.

For example, given the array [2, 4, 6, 8, 10, 2, 6, 10], return 4 and 8. The order does not matter.

Follow-up: Can you do this in linear time and constant space?
*/

use std::collections::HashMap;

// O(n) time complexity
// O(1) space complexity
pub fn appear_once(data: &[i64]) -> Vec<i64> {
    let mut xor = 0;
    for &num in data {
        xor ^= num;
    }

    let mut mask = 1;
    while xor & mask == 0 {
        mask <<= 1;
    }

    let mut a = 0;
    let mut b = 0;
    for &num in data {
        if num & mask == 0 {
            a ^= num;
        } else {
            b ^= num;
        }
    }

    vec![a, b]
}

// O(n^2) time complexity
// O(n) space complexity
pub fn appear_once1(data: &[i64]) -> Vec<i64> {
    let mut result = vec![];
    for i in 0..data.len() {
        let mut count = 0;
        for j in 0..data.len() {
            if data[i] == data[j] {
                count += 1;
            }
        }
        if count == 1 {
            result.push(data[i]);
        }
    }
    result
}

// O(n) time complexity
// O(n) space complexity
pub fn appear_once2(data: &[i64]) -> Vec<i64> {
    let mut result = vec![];
    let mut counts = HashMap::new();

    for &num in data {
        *counts.entry(num).or_insert(0) += 1;
    }

    for (num, count) in counts {
        if count == 1 {
            result.push(num);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_appear_once() {
        let data = vec![2, 4, 6, 8, 10, 2, 6, 10];
        let mut result = appear_once(&data);
        result.sort();
        assert_eq!(result, vec![4, 8]);
    }

    #[test]
    fn test_appear_once1() {
        let data = vec![2, 4, 6, 8, 10, 2, 6, 10];
        let mut result = appear_once1(&data);
        result.sort();
        assert_eq!(result, vec![4, 8]);
    }

    #[test]
    fn test_appear_once2() {
        let data = vec![2, 4, 6, 8, 10, 2, 6, 10];
        let mut result = appear_once2(&data);
        result.sort();
        assert_eq!(result, vec![4, 8]);
    }
}
