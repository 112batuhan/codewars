#![allow(dead_code)]

use std::sync::mpsc::sync_channel;

mod solutions;

fn main() {
    let small = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
    solutions::pyramid_slide_down::longest_slide_down(&small);
}
