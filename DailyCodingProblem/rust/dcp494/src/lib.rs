// This problem was asked by Facebook.
//
// Given a circular array, compute its maximum subarray sum in O(n) time. A subarray can be empty,
// and in this case the sum is 0.
//
// For example, given [8, -1, 3, 4], return 15 as we choose the numbers 3, 4, and 8 where the 8 is
// obtained from wrapping around.
//
// Given [-4, 5, 1, 0], return 6 as we choose the numbers 5 and 1.

pub fn max_circular_sum(arr: &[i32]) -> i32 {
    let mut max_sum = 0;
    let mut min_sum = 0;
    let mut total_sum = 0;
    let mut max_ending_here = 0;
    let mut min_ending_here = 0;

    for &num in arr.iter() {
        total_sum += num;
        max_ending_here = std::cmp::max(num, max_ending_here + num);
        min_ending_here = std::cmp::min(num, min_ending_here + num);
        max_sum = std::cmp::max(max_sum, max_ending_here);
        min_sum = std::cmp::min(min_sum, min_ending_here);
    }

    if max_sum > 0 {
        std::cmp::max(max_sum, total_sum - min_sum)
    } else {
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let result = max_circular_sum(&[8, -1, 3, 4]);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_example2() {
        let result = max_circular_sum(&[-4, 5, 1, 0]);
        assert_eq!(result, 6);
    }
}
