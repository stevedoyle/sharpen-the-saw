// This problem was asked by Palantir.
//
// Write a program that checks whether an integer is a palindrome. For example, 121 is a
// palindrome, as well as 888. 678 is not a palindrome. Do not convert the integer into a string.
//

pub fn is_palindrome(num: u64) -> bool {
    let mut rev = 0;
    let mut temp = num;
    while temp != 0 {
        let digit = temp % 10;
        rev = rev * 10 + digit;
        temp = temp / 10;
    }
    num == rev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(121));
        assert!(is_palindrome(1221));
        assert!(!is_palindrome(123));
    }
}
