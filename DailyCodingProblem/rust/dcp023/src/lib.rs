/// You are given an M by N matrix consisting of booleans that represents a
/// board. Each True boolean represents a wall. Each False boolean
/// represents a tile you can walk on.
///
/// Given this matrix, a start coordinate, and an end coordinate, return the
/// minimum number of steps required to reach the end coordinate from the
/// start. If there is no possible path, then return null. You can move up,
/// left, down, and right. You cannot move through walls. You cannot wrap
/// around the edges of the board.
///
/// For example, given the following board:
///
/// [[f, f, f, f],
/// [t, t, f, t],
/// [f, f, f, f],
/// [f, f, f, f]]
///
/// and start = (3, 0) (bottom left) and end = (0, 0) (top left), the
/// minimum number of steps required to reach the end is 7, since we would
/// need to go through (1, 2) because there is a wall everywhere else on the
/// second row.
use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Coordinate {
    row: i32,
    col: i32,
}

struct Step {
    coord: Coordinate,
    count: i32,
}

pub fn count_steps(board: &[Vec<bool>], start: &Coordinate, end: &Coordinate) -> i32 {
    // Traverse the board, starting at the start co-ordinate, using a deque to
    // track available moves to be considered. Each deque entry is the
    // co-ordinate and a count of steps that it took to get to that co-ordinate.
    // A set will be used to ensure that each board co-ordinate is only visited
    // once. Since there may be multiple solutions, we find them all and select
    // the solution with the smallest move count.
    let mut visited: HashMap<Coordinate, bool> = HashMap::new();
    let mut queue: VecDeque<Step> = VecDeque::new();
    queue.push_back(Step {
        coord: *start,
        count: 0,
    });
    let mut solutions: Vec<i32> = Vec::new();

    while !queue.is_empty() {
        let step = queue.pop_front().unwrap();
        if step.coord == *end {
            solutions.push(step.count);
            continue;
        }
        visited.insert(step.coord, true);
        let neighbours = get_valid_neighbours(board, &step.coord);
        for neigh in neighbours {
            if !visited.contains_key(&neigh) {
                queue.push_back(Step {
                    coord: neigh,
                    count: step.count + 1,
                });
            }
        }
    }

    let min_val = solutions.iter().min();
    match min_val {
        Some(count) => *count,
        None => -1,
    }
}

fn get_valid_neighbours(board: &[Vec<bool>], coord: &Coordinate) -> Vec<Coordinate> {
    let mut neighbours: Vec<Coordinate> = Vec::new();
    let candidates: Vec<Coordinate> = vec![
        Coordinate {
            row: coord.row,
            col: coord.col - 1,
        },
        Coordinate {
            row: coord.row - 1,
            col: coord.col,
        },
        Coordinate {
            row: coord.row + 1,
            col: coord.col,
        },
        Coordinate {
            row: coord.row,
            col: coord.col + 1,
        },
    ];
    for candidate in candidates {
        if valid_board_corodinate(board, &candidate) {
            neighbours.push(candidate);
        }
    }
    neighbours
}

fn valid_board_corodinate(board: &[Vec<bool>], coord: &Coordinate) -> bool {
    if coord.row < 0 || coord.row as usize >= board.len() {
        return false;
    }
    if coord.col < 0 || coord.col as usize >= board[0].len() {
        return false;
    }
    !board[coord.row as usize][coord.col as usize]
}

#[cfg(test)]
mod tests {
    use super::*; // Import names from outer scope

    #[test] // Attribute identifies test function
    fn test_count_steps() {
        let board = vec![
            vec![false, false, false, false],
            vec![true, true, false, true],
            vec![false, false, false, false],
            vec![false, false, false, false],
        ];
        let start = Coordinate { row: 3, col: 0 };
        let end = Coordinate { row: 0, col: 0 };
        let want = 7;
        assert_eq!(count_steps(&board, &start, &end), want);
    }
}
