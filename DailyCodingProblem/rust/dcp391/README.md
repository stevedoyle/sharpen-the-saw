# Problem 391

## The Problem

Difficulty: Hard

This problem was asked by Facebook.

We have some historical clickstream data gathered from our site anonymously using cookies. The histories contain URLs that users have visited in chronological order.

Write a function that takes two users' browsing histories as input and returns the longest contiguous sequence of URLs that appear in both.

For example, given the following two users' histories:

```
user1 = ['/home', '/register', '/login', '/user', '/one', '/two']
user2 = ['/home', '/red', '/login', '/user', '/one', '/pink']
```

You should return the following:

```
['/login', '/user', '/one']
```

## The Solution

This can be solved by *sliding* one array past the other and at each point, counting the number of matching entries within the overlapping regions of both arrays.

This solution results in a worst case complexity of O(n^3) which is made up of O(n^2) for the sliding portion and O(n) for the comparison at each overlap.

```text

Iteration 1:
                    +---+---+---+---+---+
                    | 0 | 1 | 2 | 3 | 4 |
                    +---+---+---+---+---+
    +---+---+---+---+---+
    | a | b | c | d | e |
    +---+---+---+---+---+
                    |<->|

Iteration 2:
                +---+---+---+---+---+
                | 0 | 1 | 2 | 3 | 4 |
                +---+---+---+---+---+
    +---+---+---+---+---+
    | a | b | c | d | e |
    +---+---+---+---+---+
                |<----->|

Iteration 3:
            +---+---+---+---+---+
            | 0 | 1 | 2 | 3 | 4 |
            +---+---+---+---+---+
    +---+---+---+---+---+
    | a | b | c | d | e |
    +---+---+---+---+---+
            |<--------->|

Iteration 4:
        +---+---+---+---+---+
        | 0 | 1 | 2 | 3 | 4 |
        +---+---+---+---+---+
    +---+---+---+---+---+
    | a | b | c | d | e |
    +---+---+---+---+---+
        |<------------->|

Iteration 5:
    +---+---+---+---+---+
    | 0 | 1 | 2 | 3 | 4 |
    +---+---+---+---+---+
    +---+---+---+---+---+
    | a | b | c | d | e |
    +---+---+---+---+---+
    |<----------------->|

Iteration 6:
    +---+---+---+---+---+
    | 0 | 1 | 2 | 3 | 4 |
    +---+---+---+---+---+
        +---+---+---+---+---+
        | a | b | c | d | e |
        +---+---+---+---+---+
        |<------------->|

Iteration 7:
    +---+---+---+---+---+
    | 0 | 1 | 2 | 3 | 4 |
    +---+---+---+---+---+
            +---+---+---+---+---+
            | a | b | c | d | e |
            +---+---+---+---+---+
            |<--------->|

Iteration 8:
    +---+---+---+---+---+
    | 0 | 1 | 2 | 3 | 4 |
    +---+---+---+---+---+
                +---+---+---+---+---+
                | a | b | c | d | e |
                +---+---+---+---+---+
                |<----->|


Iteration 9:
    +---+---+---+---+---+
    | 0 | 1 | 2 | 3 | 4 |
    +---+---+---+---+---+
                    +---+---+---+---+---+
                    | a | b | c | d | e |
                    +---+---+---+---+---+
                    |<->|

```