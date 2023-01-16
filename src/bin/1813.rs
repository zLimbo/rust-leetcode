struct Solution {}

impl Solution {

    pub fn are_sentences_similar(s1: String, s2: String) -> bool {
        let mut ws1: Vec<_> = s1.split(' ').collect();
        let mut ws2: Vec<_> = s2.split(' ').collect();
        if ws1.len() < ws2.len() {
            let tmp = ws1;
            ws1 = ws2;
            ws2 = tmp;
        }
        let mut i = 0;
        let (n, m) = (ws1.len(), ws2.len());
        while i < ws2.len() {
            if ws1[i] != ws2[i] {
                break;
            }
            i += 1;
        }
        while i < ws2.len() {
            if ws1[n - (m - i)] != ws2[i] {
                return false;
            }
            i += 1
        }
        true
    }
}

fn main() {

}