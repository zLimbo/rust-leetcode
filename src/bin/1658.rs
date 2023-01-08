// You are given an integer array nums and an integer x. In one operation, you can either remove the leftmost or the rightmost element from the array nums and subtract its value from x. Note that this modifies the array for future operations.

// Return the minimum number of operations to reduce x to exactly 0 if it is possible, otherwise, return -1.

//  

// Example 1:

// Input: nums = [1,1,4,2,3], x = 5
// Output: 2
// Explanation: The optimal solution is to remove the last two elements to reduce x to zero.
// Example 2:

// Input: nums = [5,6,7,8,9], x = 4
// Output: -1
// Example 3:

// Input: nums = [3,2,20,1,1,3], x = 10
// Output: 5
// Explanation: The optimal solution is to remove the last three elements and the first two elements (5 operations in total) to reduce x to zero.
//  

// Constraints:

// 1 <= nums.length <= 105
// 1 <= nums[i] <= 104
// 1 <= x <= 109

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/minimum-operations-to-reduce-x-to-zero
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        // let, tuple, mut, iter, min
        let (sum, mut l, mut r, mut win_sum, mut len) =
            (nums.iter().sum::<i32>() - x, 0, 0, 0, i32::MIN);
        // as
        if sum == 0 {
            return nums.len() as i32;
        }
        if sum < 0 {
            return -1;
        }
        while r < nums.len() {
            win_sum += nums[r];
            while win_sum >= sum {
                if win_sum == sum {
                    // max
                    len = len.max((r - l + 1) as i32);
                }
                win_sum -= nums[l];
                l += 1;
            }
            r += 1;
        }
        // if, return
        if len == i32::MIN {
            -1
        } else {
            nums.len() as i32 - len
        }
    }
}

fn main() {
    let nums = vec![1,1,4,2,3];
    let x = 5;
    println!("nums:{:?}\nx:{}", nums, x);
    let y = Solution::min_operations(nums, x);
    println!("y:{}", y);
}