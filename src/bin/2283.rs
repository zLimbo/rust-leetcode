struct Solution {}

impl Solution {
    // pub fn digit_count(num: String) -> bool {
    //     let num = num.as_bytes();
    //     let mut cnts = vec![0; num.len()];
    //     for ch in num {
    //         cnts[(ch - b'0') as usize] += 1
    //     }
    //     for i in 0..num.len() {
    //         if (num[i] - b'0') as usize != cnts[i] {
    //             return false
    //         }
    //     }
    //     return true
    // }

    pub fn digit_count(num: String) -> bool {
        let mut memo = [0;10];
        // as_bytes
        let num = num.as_bytes();
        // iter().for_each()
        num.iter().for_each(|c| memo[(c - b'0') as usize] += 1);
        // iter().enumerate().all()
        num.iter().enumerate().all(|(i, c)| memo[i] == (c - b'0') as i32)
    }
}

fn main() {
    
}