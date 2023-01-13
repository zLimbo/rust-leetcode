struct Solution {}

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        // chars() fold(B, F)
        
        let v1: Vec<i32> = s.chars().fold(vec![0; 26], |mut acc, c| {
            acc[c as usize - 'a' as usize] += 1;
            acc
        });
        let v2: Vec<i32> = target.chars().fold(vec![0; 26], |mut acc, c| {
            acc[c as usize - 'a' as usize] += 1;
            acc
        });
        // into_iter(), zip, filter, map, min, unwrap
        v1.into_iter()
            .zip(v2.into_iter())
            .filter(|(_, b)| *b != 0)
            .map(|(a, b)| a / b)
            .min()
            .unwrap()
    }

    pub fn rearrange_characters1(s: String, target: String) -> i32 {
        let mut a = [0; 26];
        let mut b = [0; 26];
        for c in s.bytes() {
            a[(c - b'a') as usize] += 1;
        }
        for c in target.bytes() {
            b[(c - b'a') as usize] += 1;
        }
        let mut res = i32::MAX;
        for i in 0..26 {
            if b[i] == 0 {
                continue;
            }
            res = res.min(a[i] / b[i]);
        }
        res
    }
}

fn main() {}
