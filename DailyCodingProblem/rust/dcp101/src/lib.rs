/*
Given an even number (greater than 2), return two prime numbers whose sum will
be equal to the given number.

A solution will always exist. See Goldbach’s conjecture.

Example:

Input: 4
Output: 2 + 2 = 4

If there are more than one solution possible, return the lexicographically
smaller solution.

If [a, b] is one solution with a <= b, and [c, d] is another solution with
c <= d, then
    [a, b] < [c, d] If a < c OR a==c AND b < d.
*/

pub fn get_prime_sum(val: usize) -> (usize, usize) {
    if val == 2 {
        return (1, 1);
    }

    let ps: Vec<usize> = primal::Primes::all().take_while(|p| p < &val).collect();
    for (i, a) in ps.iter().enumerate() {
        for b in ps[i..ps.len()].iter() {
            if a + b == val {
                return (*a, *b);
            }
        }
    }

    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_prime_sum() {
        assert_eq!(get_prime_sum(2), (1, 1));
        assert_eq!(get_prime_sum(4), (2, 2));
        assert_eq!(get_prime_sum(6), (3, 3));
        assert_eq!(get_prime_sum(8), (3, 5));
        assert_eq!(get_prime_sum(10), (3, 7));
        assert_eq!(get_prime_sum(12), (5, 7));
        assert_eq!(get_prime_sum(100), (3, 97));
    }
}
