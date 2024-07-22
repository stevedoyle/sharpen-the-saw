/*
This problem was asked by Airbnb.

Given an array of integers, return the largest range, inclusive, of integers
that are all included in the array.

For example, given the array [9, 6, 1, 3, 8, 10, 12, 11], return (8, 12) since
8, 9, 10, 11, and 12 are all in the array.
*/

pub fn largest_range(numbers: Vec<i32>) -> Vec<i32> {
    let mut numbers = numbers;
    numbers.sort();
    let mut max_range = 0;
    let mut range = 0;
    let mut start = 0;
    let mut end = 0;
    let mut max_start = 0;
    let mut max_end = 0;
    for (i, num) in numbers.iter().enumerate().skip(1) {
        if num - numbers[i - 1] == 1 {
            range += 1;
            end = i;
        } else if num - numbers[i - 1] == 0 {
            continue;
        } else {
            if range > max_range {
                max_range = range;
                max_start = start;
                max_end = end;
            }
            range = 0;
            start = i;
            end = i;
        }
    }
    // Check if the last range is the largest
    if range > max_range {
        max_start = start;
        max_end = end;
    }
    vec![numbers[max_start], numbers[max_end]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_range() {
        let result = largest_range(vec![9, 6, 1, 3, 8, 10, 12, 11]);
        assert_eq!(result, vec![8, 12]);
    }

    #[test]
    fn test_largest_rangei_with_repeats() {
        let result = largest_range(vec![-3, -2, -2, -1, 1, 2, 3]);
        assert_eq!(result, vec![-3, -1]);
    }
}
