// This problem was asked by Facebook.
//
// Given an N by N matrix, rotate it by 90 degrees clockwise.
//
// For example, given the following matrix:
//
// [[1, 2, 3],
//  [4, 5, 6],
//  [7, 8, 9]]
// you should return:
//
// [[7, 4, 1],
//  [8, 5, 2],
//  [9, 6, 3]]
// Follow-up: What if you couldn't use any extra space?

pub type Matrix = Vec<Vec<i32>>;

pub fn rotate(matrix: &Matrix) -> Matrix {
    let n_rows = matrix.len();
    let n_cols = matrix[0].len();
    let mut matrix = matrix.clone();

    for i in 0..n_rows {
        for j in i..n_cols {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }

    for i in 0..n_rows {
        for j in 0..n_cols / 2 {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[i][n_cols - 1 - j];
            matrix[i][n_cols - 1 - j] = temp;
        }
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        let result = rotate(&matrix);
        assert_eq!(result, expected);
    }
}
