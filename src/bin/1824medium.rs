use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        *obstacles
            .iter()
            .fold([1, 0, 1], |mut acc, x| {
                if *x != 0 {
                    acc[*x as usize - 1] = i32::MAX;
                }
                let min_val = *acc.iter().min().unwrap() + 1;
                acc.iter_mut().for_each(|y| *y = min_val.min(*y));
                if *x != 0 {
                    acc[*x as usize - 1] = i32::MAX;
                }
                acc
            })
            .iter()
            .min()
            .unwrap()
    }
}

fn main() {}
