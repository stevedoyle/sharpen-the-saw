/*
MegaCorp wants to give bonuses to its employees based on how many lines of codes they have written.
They would like to give the smallest positive amount to each worker consistent with the constraint
that if a developer has written more lines of code than their neighbor, they should receive more
money.

Given an array representing a line of seats of employees at MegaCorp, determine how much each one
should get paid.

For example, given [10, 40, 200, 1000, 60, 30], you should return [1, 2, 3, 4, 2, 1]
*/

pub fn bonus(seats: Vec<u64>) -> Vec<u64> {
    let mut bonuses = vec![1; seats.len()];

    for i in 1..seats.len() {
        if seats[i] > seats[i - 1] {
            bonuses[i] = bonuses[i - 1] + 1;
        }
    }

    for i in (0..seats.len() - 1).rev() {
        if seats[i] > seats[i + 1] {
            bonuses[i] = bonuses[i].max(bonuses[i + 1] + 1);
        }
    }

    bonuses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let seats = vec![10, 40, 200, 1000, 60, 30];
        assert_eq!(bonus(seats), vec![1, 2, 3, 4, 2, 1]);
    }

    #[test]
    fn test_alternating() {
        let seats = vec![10, 40, 200, 1000, 60, 100, 30];
        assert_eq!(bonus(seats), vec![1, 2, 3, 4, 1, 2, 1]);
    }
}
