use std::collections::HashMap;

struct Solution {}

impl Solution {
    

    pub fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32 {
        const WIDTH: i32 = 5;
        const WIDTH_MASK: i64 = (1 << WIDTH) - 1;

        let cnt = groups.iter().fold([0; 10], |mut acc, x| {
            acc[(*x % batch_size) as usize] += 1;
            acc
        });
        let mut start = 0 as i64;
        let mut i = (batch_size - 1) as usize;
        while i > 0 {
            start = (start << WIDTH) | cnt[i] as i64;
            i -= 1;
        }
        start <<= WIDTH;

        let mut memo:HashMap<i64, i32> = HashMap::new();

        fn dfs(mask: i64, batch_size: i32, memo: &mut HashMap<i64, i32>) -> i32 {
            if mask == 0 {
                return 0;
            }
            if memo.contains_key(&mask) {
                return memo[&mask];
            }
            let mut total = 0;
            for i in 1..batch_size {
                total += ((mask >> (i * WIDTH)) & WIDTH_MASK) * i as i64;
            }
            let total = total as i32;
            let mut best = 0;
            for i in 1..batch_size {
                let amount = (mask >> (i * WIDTH)) & WIDTH_MASK;
                if amount > 0 {
                    let mut res = dfs(mask - (1 << (i * WIDTH)), batch_size, memo);
                    if (total - i) % batch_size == 0 {
                        res += 1;
                    }
                    best = best.max(res);
                }
            }
            memo.insert(mask, best);
            best
        }

        dfs(start, batch_size, &mut memo) + cnt[0]
    }
}

fn main() {
    let res = Solution::max_happy_groups(3, vec![1, 2, 3, 4, 5, 6]);
    println!("res = {res}");
}
