// This problem was asked by Google.
//
// You are writing an AI for a 2D map game. You are somewhere in a 2D grid, and there are coins
// strewn about over the map.
//
// Given the position of all the coins and your current position, find the closest coin to you in
// terms of Manhattan distance. That is, you can move around up, down, left, and right, but not
// diagonally. If there are multiple possible closest coins, return any of them.
//
// For example, given the following map, where you are x, coins are o, and empty spaces are . (top
// left is 0, 0):
//
// ---------------------
// | . | . | x | . | o |
// ---------------------
// | o | . | . | . | . |
// ---------------------
// | o | . | . | . | o |
// ---------------------
// | . | . | o | . | . |
// ---------------------
// return (0, 4), since that coin is closest. This map would be represented in our question as:
//
// Our position: (0, 2) Coins: [(0, 4), (1, 0), (2, 0), (3, 2)]

// A point in 2D space
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

pub fn closest_coin(our_pos: Point, coins: &[Point]) -> Option<Point> {
    if coins.is_empty() {
        return None;
    }

    let mut closest_coin = &coins[0];
    let mut closest_distance = distance(&our_pos, closest_coin);

    for coin in coins.iter() {
        let distance = distance(&our_pos, coin);
        if distance < closest_distance {
            closest_distance = distance;
            closest_coin = coin;
        }
    }

    Some(*closest_coin)
}

fn distance(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let coins = vec![
            Point { x: 0, y: 4 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 2 },
        ];
        let result = closest_coin(Point { x: 0, y: 2 }, &coins);
        assert_eq!(result, Some(Point { x: 0, y: 4 }));
    }

    #[test]
    fn no_coins() {
        let coins = vec![];
        let result = closest_coin(Point { x: 0, y: 2 }, &coins);
        assert_eq!(result, None);
    }
}
