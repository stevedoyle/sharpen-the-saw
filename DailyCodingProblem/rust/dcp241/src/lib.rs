// This problem was asked by Palantir.
//
// In academia, the h-index is a metric used to calculate the impact of a researcher's papers. It is
// calculated as follows:
//
// A researcher has index h if at least h of her N papers have h citations each. If there are
// multiple h satisfying this formula, the maximum is chosen.
//
// For example, suppose N = 5, and the respective citations of each paper are [4, 3, 0, 1, 5]. Then
// the h-index would be 3, since the researcher has 3 papers with at least 3 citations.
//
// Given a list of paper citations of a researcher, calculate their h-index.

pub fn cacl_h_index(citations: &[usize]) -> usize {
    let mut sorted_citations = citations.to_owned();
    sorted_citations.sort_by(|a, b| b.cmp(a));

    for (i, cit) in sorted_citations.iter().enumerate() {
        if i >= *cit {
            return i;
        }
    }
    *sorted_citations.first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_h_index() {
        let result = cacl_h_index(&[4, 3, 0, 1, 5]);
        assert_eq!(result, 3);
        let result = cacl_h_index(&[4, 3, 0, 4, 5]);
        assert_eq!(result, 3);
        let result = cacl_h_index(&[4, 3, 4, 4, 5]);
        assert_eq!(result, 4);
    }
}
