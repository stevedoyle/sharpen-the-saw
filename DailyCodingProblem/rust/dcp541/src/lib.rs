// This problem was asked by Amazon.
//
// Run-length encoding is a fast and simple method of encoding strings. The basic idea is to
// represent repeated successive characters as a single count and character. For example, the
// string "AAAABBBCCDAA" would be encoded as "4A3B2C1D2A".
//
// Implement run-length encoding and decoding. You can assume the string to be encoded have no
// digits and consists solely of alphabetic characters. You can assume the string to be decoded is
// valid.

pub fn encode(input: &str) -> String {
    let mut result = String::new();
    let mut count = 1;
    let mut prev_char = input.chars().next().unwrap();

    for c in input.chars().skip(1) {
        if c == prev_char {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push(prev_char);
            count = 1;
            prev_char = c;
        }
    }
    result.push_str(&count.to_string());
    result.push(prev_char);
    result
}

pub fn decode(input: &str) -> String {
    let mut result = String::new();
    let mut count = 0;

    for c in input.chars() {
        if c.is_digit(10) {
            count = count * 10 + c.to_digit(10).unwrap();
        } else {
            for _ in 0..count {
                result.push(c);
            }
            count = 0;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode("AAAABBBCCDAA"), "4A3B2C1D2A");
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("4A3B2C1D2A"), "AAAABBBCCDAA");
    }
}
