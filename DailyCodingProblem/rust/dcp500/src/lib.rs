// You are given an M by N matrix consisting of booleans that represents a board. Each True boolean
// represents a wall. Each False boolean represents a tile you can walk on.
//
// Given this matrix, a start coordinate, and an end coordinate, return the minimum number of steps
// required to reach the end coordinate from the start. If there is no possible path, then return
// null. You can move up, left, down, and right. You cannot move through walls. You cannot wrap
// around the edges of the board.
//
// For example, given the following board:
//
// [[f, f, f, f],
// [t, t, f, t],
// [f, f, f, f],
// [f, f, f, f]]
//
// and start = (3, 0) (bottom left) and end = (0, 0) (top left), the minimum number of steps
// required to reach the end is 7, since we would need to go through (1, 2) because there is a wall
// everywhere else on the second row.

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

pub fn steps(board: &[Vec<bool>], start: Point, end: Point) -> u64 {
    let nrows = board.len();
    let ncols = board[0].len();
    let mut working_board = vec![vec![0; ncols]; nrows];

    for row in 0..nrows {
        for col in 0..ncols {
            if !board[row][col] {
                working_board[row][col] = -1;
            }
        }
    }

    let mut queue = vec![];
    queue.push(start);
    working_board[start.x as usize][start.y as usize] = 0;

    while !queue.is_empty() {
        let current = queue.remove(0);
        let current_steps = working_board[current.x as usize][current.y as usize];

        if current == end {
            return current_steps as u64;
        }

        let neighbors = vec![
            Point::new(current.x + 1, current.y),
            Point::new(current.x - 1, current.y),
            Point::new(current.x, current.y + 1),
            Point::new(current.x, current.y - 1),
        ];

        for neighbor in neighbors {
            if neighbor.x < 0
                || neighbor.x >= nrows.try_into().unwrap()
                || neighbor.y < 0
                || neighbor.y >= ncols.try_into().unwrap()
            {
                continue;
            }

            if working_board[neighbor.x as usize][neighbor.y as usize] == -1 {
                working_board[neighbor.x as usize][neighbor.y as usize] = current_steps + 1;
                queue.push(neighbor);
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let board = vec![
            vec![false, false, false, false],
            vec![true, true, false, true],
            vec![false, false, false, false],
            vec![false, false, false, false],
        ];
        let result = steps(&board, Point::new(3, 0), Point::new(0, 0));
        assert_eq!(result, 7);
    }
}
