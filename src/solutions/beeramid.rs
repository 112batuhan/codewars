//https://www.codewars.com/kata/51e04f6b544cf3f6550000c1/train/rust
//kyu 5

//first solution
fn beeramid(bonus: i32, price: f32) -> usize {
    let mut level: i32 = 1;
    let mut total: i32 = 0;
    let max_cans: f32 = bonus as f32 / price;
    let max_cans: i32 = max_cans as i32;
    loop {
        let current_cans_in_level = level.pow(2);
        total += current_cans_in_level;
        if total > max_cans {
            return level as usize - 1;
        }
        level += 1;
    }
}

//answer with iterators
fn _beeramid(bonus: i32, price: f32) -> usize {
    let max_count = (bonus as f32 / price) as i32;
    (1..)
        .scan(0, |acc, x| {
            *acc += x * x;
            Some(*acc)
        })
        .take_while(|&x| x <= max_count)
        .count()
}

#[cfg(test)]
mod tests {
    use super::beeramid;

    #[test]
    fn sample_tests() {
        assert_eq!(beeramid(9, 2.0), 1);
        assert_eq!(beeramid(10, 2.0), 2);
        assert_eq!(beeramid(11, 2.0), 2);
        assert_eq!(beeramid(21, 1.5), 3);
        assert_eq!(beeramid(454, 5.0), 5);
        assert_eq!(beeramid(455, 5.0), 6);
        assert_eq!(beeramid(4, 4.0), 1);
        assert_eq!(beeramid(3, 4.0), 0);
        assert_eq!(beeramid(0, 4.0), 0);
        assert_eq!(beeramid(-1, 4.0), 0);
    }
}
