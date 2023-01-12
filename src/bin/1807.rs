use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        // HashMap, map(), collect()
        let hash_map: HashMap<_, _> = knowledge
            .iter()
            .map(|x| (x[0].as_str(), x[1].as_str()))
            .collect();
        let mut s = s;
        // with_capacity
        let mut res = String::with_capacity(s.len());
        let (mut l, mut r) = (0, 0);
        // push
        s.push('(');
        // bytes()
        for c in s.bytes() {
            // match
            match c {
                b'(' => {
                    // push_str(), l..r
                    res.push_str(&s[l..r]);
                    r += 1;
                    l = r;
                }
                b')' => {
                    // if let Some(x) = xxx
                    if let Some(x) = hash_map.get(&s[l..r]) {
                        res.push_str(x);
                    } else {
                        res.push('?');
                    }
                    r += 1;
                    l = r;
                }
                _ => r += 1
            }
        }
        res
    }
}

fn main() {

}