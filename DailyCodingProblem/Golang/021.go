package dailycodingproblem

import (
	"math"
	"sort"
)

type Interval struct {
	start int
	end   int
}

func rooms_needed(intervals []Interval) int {
	rooms := 0
	max_rooms := 0
	start_times := make([]int, len(intervals))
	end_times := make([]int, len(intervals))
	for idx, ival := range intervals {
		start_times[idx] = ival.start
		end_times[idx] = ival.end
	}

	sort.Ints(start_times)
	sort.Ints(end_times)

	num_intervals := len(intervals)
	start_idx := 0
	end_idx := 0

	for (start_idx < num_intervals) && (end_idx < num_intervals) {
		if start_times[start_idx] < end_times[end_idx] {
			start_idx += 1
			rooms += 1
		} else {
			end_idx += 1
			rooms -= 1
		}
		max_rooms = int(math.Max(float64(max_rooms), float64(rooms)))
	}
	return max_rooms
}
