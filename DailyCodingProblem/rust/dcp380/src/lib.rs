// This problem was asked by Nextdoor.
//
//Implement integer division without using the division operator. Your function should return a
//tuple of (dividend, remainder) and it should take two numbers, the product and divisor.
//
// For example, calling divide(10, 3) should return (3, 1) since the divisor is 3 and the remainder
// is 1.
//
// Bonus: Can you do it in O(log n) time?

pub fn divide(numerator: i64, denominator: i64) -> (i64, i64) {
    let mut dividend = 0;
    let mut remainder = numerator.abs();
    while remainder >= denominator.abs() {
        remainder -= denominator;
        dividend += 1;
    }
    if numerator < 0 && denominator > 0 || numerator > 0 && denominator < 0 {
        dividend = -dividend;
    }
    (dividend, remainder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = divide(10, 3);
        assert_eq!(result, (3, 1));
    }

    #[test]
    fn negative_value() {
        let result = divide(-10, 3);
        assert_eq!(result, (-3, 1));
    }
}
