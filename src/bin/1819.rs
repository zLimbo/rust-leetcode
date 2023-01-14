struct Solution {}


impl Solution {

    

    pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        let maxn = *nums.iter().max().unwrap() as usize;
        let has: Vec<_> = nums.iter().fold(vec![false; maxn + 1], | mut acc, x | {
            acc[*x as usize] = true;
            acc
        });
        let mut res = has.iter().filter(|x| **x).count() as i32;
        
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 { a } else { gcd(b, a % b) }
        }

        for i in 1..=(maxn / 3) {
            if has[i] { continue }
            let mut g = 0;
            let mut j = i + i;
            while j <= maxn {
                if has[j] {
                    g = gcd(j as i32, g);
                    if g == i as i32 {
                        res += 1;
                        break;
                    }
                }
                j += i;
            }
        }
        res
    }
}


fn main() {

}