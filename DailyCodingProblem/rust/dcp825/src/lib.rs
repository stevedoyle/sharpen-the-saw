// Given a sorted list of integers, square the elements and give the output in sorted order.
//
// For example, given [-9, -2, 0, 2, 3], return [0, 4, 4, 9, 81].

pub fn square_and_sort(arr: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = arr.iter().map(|&x| x * x).collect();
    result.sort_unstable();
    result
}

pub fn square_and_sort_in_place(arr: &mut [i32]) {
    for x in arr.iter_mut() {
        *x = *x * *x;
    }
    arr.sort_unstable();
}

pub fn square_and_sort_two_pointer(arr: &[i32]) -> Vec<i32> {
    let n = arr.len();
    let mut result = vec![0; n];
    let (mut left, mut right) = (0, n - 1);
    let mut pos = n - 1;

    while left <= right {
        if arr[left].abs() > arr[right].abs() {
            result[pos] = arr[left] * arr[left];
            left += 1;
        } else {
            result[pos] = arr[right] * arr[right];
            if right == 0 {
                break;
            } // Prevent underflow
            right -= 1;
        }
        if pos == 0 {
            break;
        } // Prevent underflow
        pos -= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = square_and_sort(&[-9, -2, 0, 2, 3]);
        assert_eq!(result, [0, 4, 4, 9, 81]);
    }

    #[test]
    fn it_works_in_place() {
        let mut arr = [-9, -2, 0, 2, 3];
        square_and_sort_in_place(&mut arr);
        assert_eq!(arr, [0, 4, 4, 9, 81]);
    }

    #[test]
    fn it_works_in_place_two_pointer() {
        let mut arr = [-9, -2, 0, 2, 3];
        let result = square_and_sort_two_pointer(&mut arr);
        assert_eq!(result, [0, 4, 4, 9, 81]);
    }
}
