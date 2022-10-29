//https://www.codewars.com/kata/52597aa56021e91c93000cb0/train/rust
//kyu 5

//first solution
use std::collections::VecDeque;
fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut new_vec: VecDeque<u8> = VecDeque::new();
    for value in arr.into_iter().rev() {
        if *value == 0 {
            new_vec.push_back(*value);
        } else {
            new_vec.push_front(*value);
        }
    }
    let new_vec: Vec<u8> = Vec::from(new_vec);
    new_vec
}

//iterator solution
use std::iter;
fn move_zeros_2(arr: &[u8]) -> Vec<u8> {
    arr.iter()
        .cloned()
        .filter(|&x| x != 0)
        .chain(iter::repeat(0))
        .take(arr.len())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::move_zeros;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[u8], expected: &[u8]) {
        let actual = move_zeros(a);
        assert!(
            actual == expected,
            "With arr = {a:?}\nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest(
            &[1, 2, 0, 1, 0, 1, 0, 3, 0, 1],
            &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0],
        );
        dotest(
            &[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9],
            &[9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );
        dotest(&[0, 0], &[0, 0]);
        dotest(&[0], &[0]);
        dotest(&[], &[]);
    }
}
