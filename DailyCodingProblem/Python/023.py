"""
You are given an M by N matrix consisting of booleans that represents a
board. Each True boolean represents a wall. Each False boolean
represents a tile you can walk on.

Given this matrix, a start coordinate, and an end coordinate, return the
minimum number of steps required to reach the end coordinate from the
start. If there is no possible path, then return null. You can move up,
left, down, and right. You cannot move through walls. You cannot wrap
around the edges of the board.

For example, given the following board:

[[f, f, f, f],
[t, t, f, t],
[f, f, f, f],
[f, f, f, f]]

and start = (3, 0) (bottom left) and end = (0, 0) (top left), the
minimum number of steps required to reach the end is 7, since we would
need to go through (1, 2) because there is a wall everywhere else on the
second row.
"""

from collections import deque
import unittest


def valid_board_coordinate(board, pos):
    row, col = pos
    if row < 0 or row >= len(board):
        return False
    if col < 0 or col >= len(board[0]):
        return False
    return not board[row][col]


def get_valid_neighbours(board, pos):
    row, col = pos
    neighbours = []
    for candidate in [(row, col - 1), (row - 1, col), (row + 1, col), (row, col + 1)]:
        if valid_board_coordinate(board, candidate):
            neighbours.append(candidate)
    return neighbours


def count_steps(board, start, end):
    """
    Traverse the board, starting at the start co-ordinate, using a deque
    to track available moves to be considered. Each deque entry is the
    co-ordinate and a count of steps that it took to get to that
    co-ordinate. A set will be used to ensure that each board
    co-ordinate is only visited once. Since there may be multiple
    solutions, we find them all and select the solution with the
    smallest move count.
    """
    visited = set()
    queue = deque([(start, 0)])
    solutions = []
    while queue:
        pos, count = queue.popleft()
        if pos == end:
            solutions.append(count)
            continue
        visited.add(pos)
        neighbours = get_valid_neighbours(board, pos)
        for n in neighbours:
            if n not in visited:
                queue.extend([(n, count + 1)])

    if len(solutions) == 0:
        return -1
    return min(solutions)


class Test023CountSteps(unittest.TestCase):
    def testGivenProblem(self):
        board = [
            [False, False, False, False],
            [True, True, False, True],
            [False, False, False, False],
            [False, False, False, False],
        ]
        start = (3, 0)
        end = (0, 0)
        want = 7
        self.assertEqual(want, count_steps(board, start, end))


if __name__ == "__main__":
    unittest.main()
