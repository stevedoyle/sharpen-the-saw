/* Problem:

Find the maximum of two numbers without using any if-else statements, branching, or
direct comparisons.
 */

pub fn get_max(a: i32, b: i32) -> i32 {
    let c = a - b;
    let k = (c >> 31) & 1;
    a - (k * c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_max() {
        assert_eq!(3, get_max(-3, 3));
        assert_eq!(3, get_max(3, -3));
        assert_eq!(4, get_max(4, 2));
        assert_eq!(4, get_max(2, 4));
        assert_eq!(-1, get_max(-1, -2));
    }
}
