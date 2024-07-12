// This problem was asked by WeWork.
//
// You are given an array of integers representing coin denominations and a total amount of money.
// Write a function to compute the fewest number of coins needed to make up that amount. If it is
// not possible to make that amount, return null.
//
// For example, given an array of [1, 5, 10] and an amount 56, return 7 since we can use 5 dimes, 1
// nickel, and 1 penny.
//
// Given an array of [5, 8] and an amount 15, return 3 since we can use 5 5-cent coins.

pub fn least_coins(coins: &[u64], amount: u64) -> u64 {
    if amount == 0 {
        return 0;
    }

    let mut result = u64::MAX;

    for i in 0..coins.len() {
        if coins[i] <= amount {
            let sub_result = least_coins(coins, amount - coins[i]);
            if sub_result < u64::MAX {
                result = result.min(sub_result + 1);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let result = least_coins(&[1, 5, 10], 56);
        assert_eq!(result, 7);
    }

    #[test]
    fn example2() {
        let result = least_coins(&[5, 8], 15);
        assert_eq!(result, 3);
    }

    #[test]
    fn no_solution() {
        let result = least_coins(&[5, 10], 56);
        assert_eq!(result, u64::MAX);
    }
}
