/*
Spreadsheets often use this alphabetical encoding for its columns:
    "A", "B", "C", ..., "AA", "AB", ..., "ZZ", "AAA", "AAB", ....

Given a column number, return its alphabetical column id. For example, given 1, return "A".
Given 27, return "AA".
 */

pub fn col_id(col: u32) -> String {
    let mut id = String::new();
    let mut tmp = col;
    loop {
        let (div, rem) = divmod(tmp - 1, 26);
        id.push((b'A' + rem as u8) as char);
        tmp = div;
        if tmp == 0 {
            break;
        }
    }
    id.chars().rev().collect()
}

pub fn divmod(a: u32, b: u32) -> (u32, u32) {
    (a / b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_col_id() {
        assert_eq!("A", col_id(1));
        assert_eq!("B", col_id(2));
        assert_eq!("Z", col_id(26));
        assert_eq!("AA", col_id(27));
        assert_eq!("AB", col_id(28));
    }
}
