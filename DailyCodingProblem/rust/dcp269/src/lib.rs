/* This problem was asked by Microsoft.

You are given an string representing the initial conditions of some dominoes. Each element can take one of three values:

L, meaning the domino has just been pushed to the left,
R, meaning the domino has just been pushed to the right, or
., meaning the domino is standing still.
Determine the orientation of each tile when the dominoes stop falling. Note that if a domino receives a force from the left and right side simultaneously, it will remain upright.

For example, given the string .L.R....L, you should return LL.RRRLLL.

Given the string ..R...L.L, you should return ..RR.LLLL.
 */

pub fn dominoes(start: &str) -> String {
    let mut tmp = str::replace(&start, "R.L", "X");

    let modified = loop {
        let tmp1 = str::replace(&tmp, ".L", "LL");
        let tmp2 = str::replace(&tmp1, "R.", "RR");
        let tmp3 = str::replace(&tmp2, "R.L", "X");
        if tmp1 == tmp3 {
            break tmp3;
        }
        tmp = tmp3;
    };
    str::replace(&modified, "X", "R.L")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dominoes() {
        assert_eq!("LL.RRRLLL", dominoes(".L.R....L"));
        assert_eq!("..RR.LLLL", dominoes("..R...L.L"));
    }
}
