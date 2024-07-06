// This problem was asked by Facebook.
//
// Given a list of integers L, find the maximum length of a sequence of consecutive numbers that can
// be formed using elements from L.
//
// For example, given L = [5, 2, 99, 3, 4, 1, 100], return 5 as we can build a sequence [1, 2, 3, 4,
// 5] which has length 5.

pub fn max_sequence_length(seq: &[i64]) -> u64 {
    if seq.is_empty() {
        return 0;
    }

    let mut seq = seq.to_vec();
    seq.sort_unstable();

    let mut max_length = 0;
    let mut current_length = 1;
    let mut looking_for = seq[0] + 1;

    // iterate over the sorted sequence starting from the second element
    for num in seq.iter().skip(1) {
        // if the current number is not the one we are looking for, we reset the current length
        // and start looking for the next number
        if *num != looking_for {
            if current_length > max_length {
                max_length = current_length;
            }
            current_length = 1;
            looking_for = *num + 1;
            continue;
        }
        current_length += 1;
        looking_for += 1;
    }

    if current_length > max_length {
        max_length = current_length;
    }

    max_length as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = max_sequence_length(&[5, 2, 99, 3, 4, 1, 100]);
        assert_eq!(result, 5);

        let result = max_sequence_length(&[-1, -2, 0, 3, 5, 4, 7, 8]);
        assert_eq!(result, 3);
    }

    #[test]
    fn no_consecutive_numbers() {
        let result = max_sequence_length(&[-1, 3, 5, 7]);
        assert_eq!(result, 1);
    }

    #[test]
    fn empty_list() {
        let result = max_sequence_length(&[]);
        assert_eq!(result, 0);
    }
}
