/*
This problem was asked by Facebook.

We have some historical clickstream data gathered from our site anonymously using cookies. The
histories contain URLs that users have visited in chronological order.

Write a function that takes two users' browsing histories as input and returns the longest
contiguous sequence of URLs that appear in both.

For example, given the following two users' histories:

user1 = ['/home', '/register', '/login', '/user', '/one', '/two']
user2 = ['/home', '/red', '/login', '/user', '/one', '/pink']
You should return the following:

['/login', '/user', '/one']
*/

use std::cmp::min;

// Complexity: O(n^3)
pub fn longest_sequence(user1: Vec<String>, user2: Vec<String>) -> Vec<String> {
    let mut match_pos = 0;
    let mut max_match_len = 0;

    for i in 0..user1.len() {
        for j in 0..user2.len() {
            let window_len = min(user1.len() - i, user2.len() - j);
            let (longest_match_count, pos) =
                count_matches(&user1[i..i + window_len], &user2[j..j + window_len]);
            if longest_match_count > max_match_len {
                max_match_len = longest_match_count;
                match_pos = pos + i;
            }
        }
    }
    if max_match_len == 0 {
        return Vec::new();
    }
    user1[match_pos..match_pos + max_match_len].to_vec()
}

// Complexity: O(n)
fn count_matches(user1: &[String], user2: &[String]) -> (usize, usize) {
    let mut max_count = 0;
    let mut max_pos = 0;
    let mut count = 0;

    for i in 0..user1.len() {
        if user1[i] == user2[i] {
            count += 1;
        } else {
            if count > max_count {
                max_count = count;
                max_pos = i - count;
            }
            count = 0;
        }
    }
    (max_count, max_pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_391() {
        let user1 = vec![
            "/home".to_string(),
            "/register".to_string(),
            "/login".to_string(),
            "/user".to_string(),
            "/one".to_string(),
            "/two".to_string(),
        ];
        let user2 = vec![
            "/home".to_string(),
            "/red".to_string(),
            "/login".to_string(),
            "/user".to_string(),
            "/one".to_string(),
            "/pink".to_string(),
        ];
        assert_eq!(
            longest_sequence(user1, user2),
            vec![
                "/login".to_string(),
                "/user".to_string(),
                "/one".to_string()
            ]
        );
    }

    #[test]
    fn test_391_2() {
        let user1 = vec![
            "/home".to_string(),
            "/register".to_string(),
            "/login".to_string(),
            "/user".to_string(),
            "/one".to_string(),
            "/two".to_string(),
        ];
        let user2 = vec![
            "/red".to_string(),
            "/login".to_string(),
            "/user".to_string(),
            "/one".to_string(),
            "/pink".to_string(),
            "/two".to_string(),
            "/home".to_string(),
        ];
        assert_eq!(
            longest_sequence(user1, user2),
            vec![
                "/login".to_string(),
                "/user".to_string(),
                "/one".to_string(),
            ]
        );
    }
}
