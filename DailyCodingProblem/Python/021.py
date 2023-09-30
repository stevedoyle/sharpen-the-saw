"""
This problem was asked by Snapchat.

Given an array of time intervals (start, end) for classroom lectures (possibly overlapping), find the minimum number of rooms required.

For example, given [(30, 75), (0, 50), (60, 150)], you should return 2.
"""

import unittest

def rooms_needed(intervals):
    rooms = 0
    max_rooms = 0
    start_times = [interval[0] for interval in intervals]
    start_times.sort()
    end_times = [interval[1] for interval in intervals]
    end_times.sort()

    num_intervals = len(intervals)
    start_idx = 0
    end_idx = 0

    while start_idx < num_intervals and end_idx < num_intervals:
        if start_times[start_idx] < end_times[end_idx]:
            start_idx += 1
            rooms += 1
        else:
            end_idx += 1
            rooms -= 1

        max_rooms = max(max_rooms, rooms)
    return max_rooms

class Test021RoomBooking(unittest.TestCase):
    def testOverlappingIntervals(self):
        input = [(30, 75), (0, 50), (60, 150)]
        want = 2
        self.assertEqual(want, rooms_needed(input))

        input = [(30, 75), (0, 50), (40, 150)]
        want = 3
        self.assertEqual(want, rooms_needed(input))

    def testNonOverlappingIntervals(self):
        input = [(0, 40), (50, 70), (75, 100)]
        want = 1
        self.assertEqual(want, rooms_needed(input))

    def testDuplicateIntervals(self):
        input = [(0, 40), (0, 40)]
        want = 2
        self.assertEqual(want, rooms_needed(input))

if __name__ == "__main__":
    unittest.main()