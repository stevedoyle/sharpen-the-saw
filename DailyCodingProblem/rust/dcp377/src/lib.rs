//
// This problem was asked by Microsoft.
//
// Given an array of numbers arr and a window of size k, print out the median of each window of size
// k starting from the left and moving right by one position each time.
//
// For example, given the following array and k = 3:
//
// [-1, 5, 13, 8, 2, 3, 3, 1]
// Your function should print out the following:
//
// 5 <- median of [-1, 5, 13]
// 8 <- median of [5, 13, 8]
// 8 <- median of [13, 8, 2]
// 3 <- median of [8, 2, 3]
// 3 <- median of [2, 3, 3]
// 3 <- median of [3, 3, 1]
// Recall that the median of an even-sized list is the average of the two middle numbers.

pub fn sliding_median(a: &[i64], window_size: usize) -> Vec<i64> {
    a.windows(window_size)
        .map(|w| {
            let mut w = w.to_vec();
            w.sort();
            if w.len() % 2 == 0 {
                (w[w.len() / 2] + w[w.len() / 2 - 1]) / 2
            } else {
                w[w.len() / 2]
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliding_median() {
        let result = sliding_median(&[-1, 5, 13, 8, 2, 3, 3, 1], 3);
        assert_eq!(result, [5, 8, 8, 3, 3, 3,]);
    }

    #[test]
    fn test_sliding_median_even_window_size() {
        let result = sliding_median(&[-1, 5, 13, 8, 2, 3, 3, 1], 4);
        assert_eq!(result, [6, 6, 5, 3, 2]);
    }
}
