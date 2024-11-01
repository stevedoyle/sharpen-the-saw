/*
    This problem was asked by Uber.

    Write a program that determines the smallest number of perfect squares that sum up to N.

    Here are a few examples:

    Given N = 4, return 1 (4)
    Given N = 17, return 2 (16 + 1)
    Given N = 18, return 2 (9 + 9)
*/

#![feature(isqrt)]

pub fn min_squares(n: u64) -> u64 {
    let mut perfect_squares = Vec::new();
    for i in 1..(n.isqrt() + 1) {
        if i * i <= n {
            perfect_squares.push(i * i);
        }
    }

    if perfect_squares.iter().any(|x| *x == n) {
        return 1;
    }

    for p in perfect_squares.iter() {
        if perfect_squares.iter().any(|x| *x == (n - p)) {
            return 2;
        }
    }

    // Any integer can be expressed as the sum of at most four perfect squares.
    // An integer is the sum of three perfect squares if and only if it cannot be expressed in the
    // form N = 4a* (8 * b + 7), where a and b are integers.
    let mut x = n;
    while x % 4 == 0 {
        x /= 4;
    }
    if x % 8 != 4 {
        return 3;
    }

    4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numps() {
        assert_eq!(min_squares(1), 1);
        assert_eq!(min_squares(4), 1);
        assert_eq!(min_squares(17), 2);
        assert_eq!(min_squares(18), 2);
    }
}
