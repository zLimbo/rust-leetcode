struct Solution {}

impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut n = nums.len();
        while n > 1 {
            n /= 2;
            for i in 0..n {
                match i & 1 {
                    0 => nums[i] = nums[i * 2].min(nums[i * 2 + 1]),
                    1 => nums[i] = nums[i * 2].max(nums[i * 2 + 1]),
                    // panic!()
                    _ => panic!(),
                }
            }
        }
        nums[0]
    }
}

fn main() {}
