// 2185. Counting Words With a Given Prefix
// You are given an array of strings words and a string pref.

// Return the number of strings in words that contain pref as a prefix.

// A prefix of a string s is any leading contiguous substring of s.

 

// Example 1:

// Input: words = ["pay","attention","practice","attend"], pref = "at"
// Output: 2
// Explanation: The 2 strings that contain "at" as a prefix are: "attention" and "attend".
// Example 2:

// Input: words = ["leetcode","win","loops","success"], pref = "code"
// Output: 0
// Explanation: There are no strings that contain "code" as a prefix.
 

// Constraints:

// 1 <= words.length <= 100
// 1 <= words[i].length, pref.length <= 100
// words[i] and pref consist of lowercase English letters.

struct Solution {}

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        // fold, start_with
        return words
            .iter()
            .fold(0, |acc, x| acc + x.starts_with(&pref) as i32);
    }
}

fn main() {
    // &str, to_string
    let words = vec![
        "pay".to_string(),
        "attention".to_string(),
        "practice".to_string(),
        "attend".to_string(),
    ];
    let pref = "at".to_string();
    println!("words:{:?}\npref:{}", words, pref);
    let cnt = Solution::prefix_count(words, pref);
    println!("cnt:{}", cnt)
}
