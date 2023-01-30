use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        logs.iter()
            .fold(HashMap::new(), |mut map, log| {
                map.entry(log[0]).or_insert(HashSet::new()).insert(log[1]);
                map
            })
            .iter()
            .fold(vec![0; k as usize], |mut ve, (_, set)| {
                ve[set.len() - 1] += 1;
                ve
            })
    }
}

fn main() {}
