// Difficlty: Easy
//
// This problem was asked by Pivotal.
//
// Write an algorithm that finds the total number of set bits in all integers between 1 and N.

pub fn count_set_bits(n: u32) -> u32 {
    let mut count = 0;
    for i in 1..=n {
        count += i.count_ones();
    }
    count
}

pub fn count_set_bits2(n: u32) -> u32 {
    (1..=n).reduce(|acc, x| acc + x.count_ones()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_set_bits() {
        assert_eq!(count_set_bits(0), 0);
        assert_eq!(count_set_bits(1), 1);
        assert_eq!(count_set_bits(2), 2);
        assert_eq!(count_set_bits(3), 4);
        assert_eq!(count_set_bits(10), 17);
    }

    #[test]
    fn test_count_set_bits2() {
        assert_eq!(count_set_bits2(0), 0);
        assert_eq!(count_set_bits2(1), 1);
        assert_eq!(count_set_bits2(2), 2);
        assert_eq!(count_set_bits2(3), 4);
        assert_eq!(count_set_bits2(10), 17);
    }
}
