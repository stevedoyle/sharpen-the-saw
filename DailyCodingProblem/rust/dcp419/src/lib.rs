/*
This problem was asked by PagerDuty.

Given a positive integer N, find the smallest number of steps it will take to reach 1.

There are two kinds of permitted steps:

You may decrement N to N - 1.
If a * b = N, you may decrement N to the larger of a and b.
For example, given 100, you can reach 1 in five steps with the following route:
    100 -> 10 -> 9 -> 3 -> 2 -> 1.
*/

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
    fn test_get_divisors() {
        assert_eq!(get_divisors(100), vec![10, 20, 25, 50, 100]);
    }
}
