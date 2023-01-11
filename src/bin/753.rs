struct Solution {}

impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        // as
        let n = n as usize;
        let k = k as usize;
        // pow(u32)
        let kn = k.pow(n as u32) as usize;
        // vec![值; 数目]
        let mut num = vec![k as u8; kn];
        // b'0'
        let mut res = vec![b'0'; kn + n - 1];
        let mut node = 0;
        // ..
        for i in n - 1..res.len() {
            num[node] -= 1;
            res[i] = num[node] + b'0';
            node = (node * k + num[node] as usize) % (kn / k) ; 
        }
        // unsafe
        unsafe { String::from_utf8_unchecked(res)}
    }
}

fn main() {
    let (n, k) = (4, 4);
    let res = Solution::crack_safe(n, k);
    assert_eq!(res.len(), (k.pow(n as u32) + n - 1) as usize);
    println!("n:{}, k:{}, res.len:{}\nres:{}", n, k, res.len(), res);
}