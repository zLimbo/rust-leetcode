use std::collections::{BTreeMap, VecDeque}; // BTreeMap, VecDeque

struct MultiBTreeSet<T> {
    // 泛型
    key2cnt: BTreeMap<T, i32>,
}

impl<T> MultiBTreeSet<T>
where 
    T: PartialOrd + Ord + Copy, // 泛型 类型约束
{
    pub fn new() -> Self {
        return Self {
            key2cnt: BTreeMap::new(),
        };
    }

    pub fn insert(&mut self, key: T) {
        *self.key2cnt.entry(key).or_insert(0) += 1;
    }

    pub fn remove(&mut self, key: &T) -> bool {
        let need_remove = {
            if let Some(count) = self.key2cnt.get_mut(key) {
                *count -= 1;
                *count == 0
            } else {
                return false; // 从函数返回
            }
        };
        if need_remove {
            self.key2cnt.remove(key);
        }
        true
    }

    pub fn peek_first(&mut self) -> &T {
        return self.key2cnt.iter().next().unwrap().0; // iter().next().unwrap().0
    }

    pub fn peek_last(&mut self) -> &T {
        return self.key2cnt.iter().rev().next().unwrap().0; // rev()
    }

    pub fn pop_first(&mut self) -> T {
        let x = *self.peek_first();
        self.remove(&x);
        return x;
    }

    pub fn pop_last(&mut self) -> T {
        let x = *self.peek_last();
        self.remove(&x);
        return x;
    }
}

struct MKAverage {
    m: usize,
    k: usize,
    sum: i32,
    fifo: VecDeque<i32>,
    s_min: MultiBTreeSet<i32>,
    s_mid: MultiBTreeSet<i32>,
    s_max: MultiBTreeSet<i32>,
}

impl MKAverage {
    fn new(m: i32, k: i32) -> Self {
        return Self {
            m: m as usize,
            k: k as usize,
            sum: 0,
            fifo: VecDeque::with_capacity(m as usize),
            s_min: MultiBTreeSet::new(),
            s_mid: MultiBTreeSet::new(),
            s_max: MultiBTreeSet::new(),
        };
    }

    fn add_element(&mut self, num: i32) {
        self.fifo.push_back(num);
        if self.fifo.len() <= self.m {
            self.sum += num;
            self.s_mid.insert(num);
            if self.fifo.len() == self.m {
                for _ in 0..self.k {
                    let x = self.s_mid.pop_first();
                    self.s_min.insert(x);
                    self.sum -= x;
                }
                for _ in 0..self.k {
                    let x = self.s_mid.pop_last();
                    self.s_max.insert(x);
                    self.sum -= x;
                }
            }
            return;
        }

        if num < *self.s_min.peek_last() {
            let x = self.s_min.pop_last();
            self.s_mid.insert(x);
            self.sum += x;
            self.s_min.insert(num);
        } else if num > *self.s_max.peek_first() {
            let x = self.s_max.pop_first();
            self.s_mid.insert(x);
            self.sum += x;
            self.s_max.insert(num);
        } else {
            self.sum += num;
            self.s_mid.insert(num);
        }

        let x = self.fifo.pop_front().unwrap();
        if self.s_mid.remove(&x) {
            self.sum -= x;
        } else if self.s_min.remove(&x) {
            let y = self.s_mid.pop_first();
            self.s_min.insert(y);
            self.sum -= y;
        } else if self.s_max.remove(&x) {
            let y = self.s_mid.pop_last();
            self.s_max.insert(y);
            self.sum -= y;
        } else {
            panic!();
        }
    }

    fn calculate_mk_average(&self) -> i32 {
        if self.fifo.len() < self.m as usize {
            return -1;
        }
        self.sum / (self.m as i32 - self.k as i32 * 2)
    }
}

fn test1() {
    let mut obj = MKAverage::new(3, 1);
    obj.add_element(3); // current elements are [3]
    obj.add_element(1); // current elements are [3,1]
    let x = obj.calculate_mk_average(); // return -1, because m = 3 and only 2 elements exist.
    println!("{x:?}");
    obj.add_element(10); // current elements are [3,1,10]
    let x = obj.calculate_mk_average(); // The last 3 elements are [3,1,10]. // After removing smallest and largest 1 element the container will be [3]. // The average of [3] equals 3/1 = 3, return 3
    println!("{x:?}");
    obj.add_element(5); // current elements are [3,1,10,5]
    obj.add_element(5); // current elements are [3,1,10,5,5]
    obj.add_element(5); // current elements are [3,1,10,5,5,5]
    let x = obj.calculate_mk_average(); // The last 3 elements are [5,5,5].
    println!("{x:?}");
}

fn test2() {
    let mut obj = MKAverage::new(6, 1);
    obj.add_element(3);
    obj.add_element(1);
    obj.add_element(12);
    obj.add_element(5);
    obj.add_element(3);
    obj.add_element(4);
    let x = obj.calculate_mk_average();
    println!("{x:?}");
}

fn main() {
    test1();
    println!();
    test2();
}
