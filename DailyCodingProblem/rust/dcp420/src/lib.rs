/*
This problem was asked by Microsoft.

A number is considered perfect if its digits sum up to exactly 10.

Given a positive integer n, return the n-th perfect number.

For example, given 1, you should return 19. Given 2, you should return 28.
*/

// Simple solution that is fast and uses a precomputed list of perfect numbers.
// Limitation is that it only works for n <= 10.
pub fn perfect_bounded(n: u32) -> Option<u32> {
    let perfect_nums = [19, 28, 37, 46, 55, 64, 73, 82, 91, 109];
    if n == 0 || n > perfect_nums.len() as u32 {
        return None;
    }
    Some(perfect_nums[n as usize - 1])
}

// More general solution that works for any n.
pub fn perfect(n: u32) -> Option<u32> {
    if n == 0 {
        return None;
    }
    let mut count = 1;
    let mut i = 19;
    while count < n {
        i += 1;
        if sum_digits(i) == 10 {
            count += 1;
        }
    }
    Some(i)
}

fn sum_digits(n: u32) -> u32 {
    let mut sum = 0;
    let mut n = n;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_bounded() {
        assert_eq!(perfect_bounded(1), Some(19));
        assert_eq!(perfect(2), Some(28));
    }

    #[test]
    fn test_perfect_bounded_out_of_bounds() {
        assert_eq!(perfect_bounded(0), None);
        assert_eq!(perfect_bounded(20), None);
    }

    #[test]
    fn test_perfect() {
        assert_eq!(perfect(1), Some(19));
        assert_eq!(perfect(2), Some(28));
        assert_eq!(perfect(20), Some(208));
    }

    #[test]
    fn test_perfect_out_of_bounds() {
        assert_eq!(perfect(0), None);
    }

    #[test]
    fn test_sum_digits() {
        assert_eq!(sum_digits(19), 10);
        assert_eq!(sum_digits(28), 10);
        assert_eq!(sum_digits(109), 10);
        assert_eq!(sum_digits(190), 10);
    }
}
