/*
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
*/

package dailycodingproblem

import (
	"sort"

	"github.com/gammazero/deque"
)

type Coordinate struct {
	row int
	col int
}

type Step struct {
	coord Coordinate
	count int
}

func countSteps(board [][]bool, start Coordinate, end Coordinate) int {
	/*
			Traverse the board, starting at the start co-ordinate, using a deque
		    to track available moves to be considered. Each deque entry is the
		    co-ordinate and a count of steps that it took to get to that
		    co-ordinate. A set will be used to ensure that each board
		    co-ordinate is only visited once. Since there may be multiple
		    solutions, we find them all and select the solution with the
		    smallest move count.
	*/
	visited := make(map[Coordinate]bool)
	queue := deque.New[Step]()
	queue.PushBack(Step{start, 0})
	var solutions []int

	for queue.Len() > 0 {
		step := queue.PopFront()
		if step.coord == end {
			solutions = append(solutions, step.count)
			continue
		}
		visited[step.coord] = true
		neighbours := getValidNeighbours(board, step.coord)
		for _, neigh := range neighbours {
			_, ok := visited[neigh]
			if !ok {
				queue.PushBack(Step{neigh, step.count + 1})
			}
		}
	}

	if len(solutions) == 0 {
		return -1
	}
	sort.Ints(solutions)
	return solutions[0]
}

func getValidNeighbours(board [][]bool, coord Coordinate) []Coordinate {
	var neighbours []Coordinate
	candidates := []Coordinate{
		{coord.row, coord.col - 1},
		{coord.row - 1, coord.col},
		{coord.row + 1, coord.col},
		{coord.row, coord.col + 1},
	}
	for _, candidate := range candidates {
		if validBoardCoordinate(board, candidate) {
			neighbours = append(neighbours, candidate)
		}
	}
	return neighbours
}

func validBoardCoordinate(board [][]bool, coord Coordinate) bool {
	if coord.row < 0 || coord.row >= len(board) {
		return false
	}
	if coord.col < 0 || coord.col >= len(board[0]) {
		return false
	}
	return !board[coord.row][coord.col]
}
