/*
This problem was asked by PagerDuty.

Given a positive integer N, find the smallest number of steps it will take to reach 1.

There are two kinds of permitted steps:

You may decrement N to N - 1.
If a * b = N, you may decrement N to the larger of a and b.
For example, given 100, you can reach 1 in five steps with the following route:
    100 -> 10 -> 9 -> 3 -> 2 -> 1.
*/

// Dynamic programming solution
// Time complexity: O(n * sqrt(n))
// Space complexity: O(n)
pub fn steps_dynamic(n: u64) -> u64 {
    let mut distance = (0..=n).map(|i| (i as i64 - 1)).collect::<Vec<i64>>();

    for i in 1..=n {
        let sqrt_n = (i as f64).sqrt() as u64;
        for j in (1..=sqrt_n).rev() {
            if i % j == 0 {
                distance[i as usize] = distance[i as usize].min(distance[(i / j) as usize] + 1);
            }
        }
        distance[i as usize] = distance[i as usize].min(distance[(i - 1) as usize] + 1);
    }
    distance[n as usize] as u64
}

// Breadth-first search solution
// Time complexity: O(n * sqrt(n))
// Space complexity: O(n)
// This solution is slower than the dynamic programming solution
// because it has to check all divisors of a number
// and the number of divisors of a number is O(sqrt(n))
// So the time complexity is O(n * sqrt(n))
pub fn steps(n: u64) -> u64 {
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((n, 0));
    let mut visited = std::collections::HashSet::new();

    while let Some((num, steps)) = queue.pop_front() {
        visited.insert(num);

        if num == 1 {
            return steps;
        }

        let mut candidates = get_divisors(num);
        candidates.push(num - 1);
        for candidate in candidates {
            if !visited.contains(&candidate) {
                queue.push_back((candidate, steps + 1));
            }
        }
    }
    0
}

fn get_divisors(n: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    let max_divisor = (n as f64).sqrt() as u64;
    // Iterate from max_divisor down to 1
    for i in (1..=max_divisor).rev() {
        if n % i == 0 {
            divisors.push(n / i);
        }
    }
    divisors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steps() {
        assert_eq!(steps(100), 5);
    }

    #[test]
    fn test_steps_dynamic() {
        assert_eq!(steps_dynamic(100), 5);
    }

    #[test]
    fn test_get_divisors() {
        assert_eq!(get_divisors(100), vec![10, 20, 25, 50, 100]);
    }
}
