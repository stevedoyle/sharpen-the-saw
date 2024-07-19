/*
This problem was asked by Google.

You are given a 2D matrix of 1s and 0s where 1 represents land and 0 represents water.

Grid cells are connected horizontally orvertically (not diagonally). The grid is completely
surrounded by water, and there is exactly one island (i.e., one or more connected land cells).

An island is a group is cells connected horizontally or vertically, but not diagonally. There is
guaranteed to be exactly one island in this grid, and the island doesn't have water inside that
isn't connected to the water around the island. Each cell has a side length of 1.

Determine the perimeter of this island.

For example, given the following matrix:

[[0, 1, 1, 0],
[1, 1, 1, 0],
[0, 1, 1, 0],
[0, 0, 1, 0]]
Return 14.
*/

// Complexity Analysis
//  - Time complexity: O(N*M), where N is the number of rows and M is the number of columns in the grid.
//  - Space complexity: O(1).
pub fn perimeter(grid: Vec<Vec<usize>>) -> usize {
    let mut edge_count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            edge_count += perimeter_helper(&grid, row, col);
        }
    }
    edge_count
}

fn perimeter_helper(grid: &[Vec<usize>], row: usize, col: usize) -> usize {
    if row >= grid.len() || col >= grid[0].len() || grid[row][col] == 0 {
        return 0;
    }

    let mut edge_count = 0;

    // If the current cell is on the edge of the grid, add 1 to the edge count
    if row == 0 || row == grid.len() - 1 || col == 0 || col == grid[0].len() - 1 {
        edge_count += 1;
    }
    // If the current cell is not on the edge of the grid, add 1 to the edge count if any of its
    // adjacent cells are water (0)
    if row > 0 && grid[row - 1][col] == 0 {
        edge_count += 1;
    }
    if row < grid.len() - 1 && grid[row + 1][col] == 0 {
        edge_count += 1;
    }
    if col > 0 && grid[row][col - 1] == 0 {
        edge_count += 1;
    }
    if col < grid[0].len() - 1 && grid[row][col + 1] == 0 {
        edge_count += 1;
    }
    edge_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perimeter() {
        let grid = vec![
            vec![0, 1, 1, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 0],
        ];
        assert_eq!(perimeter(grid), 14);
    }
}
