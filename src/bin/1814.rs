struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        fn rev(mut x: i32) -> i32 {
            let mut y = 0;
            while x != 0 {
                y = y * 10 + x % 10;
                x /= 10;
            }
            y
        }
        let mut map: HashMap<i32, i32> = HashMap::new();
        for x in nums {
            // entry, or_insert, and_modify
            // *map.entry(x - rev(x)).or_insert(0) += 1;
            map.entry(x - rev(x)).and_modify(|it| *it += 1).or_insert(1);
        }
        let mut res = 0;
        // const
        const MOD: i64 = 1000_000_000 + 7;
        for (_, v) in map {
            let v = v as i64;
            res = (res + v * (v - 1) / 2) % MOD;
        }
        res as i32
    }
}

fn main() {}
