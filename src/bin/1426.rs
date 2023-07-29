pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn count_elements(arr: Vec<i32>) -> i32 {
        let hash_set: HashSet<i32> = arr.clone().into_iter().collect();
        let mut cnt = 0;
        arr.iter().for_each(|x| {
            if hash_set.contains(&(x + 1)) {
                cnt += 1;
            }
        });
        cnt
    }

    pub fn count_elements_another(arr: Vec<i32>) -> i32 {
        let set: HashSet<_> = arr.iter().cloned().collect();
        let mut cnt = 0;
        for a in arr {
            if set.contains(&(a + 1)) {
                cnt += 1;
            }
        }
        cnt
    }
}

fn main() {
    println!("{}", Solution::count_elements(vec![1, 1, 3, 3, 5, 5, 7, 7]));
    println!("{}", Solution::count_elements(vec![1, 2, 3, 3, 4]));
    println!("{}", Solution::count_elements(vec![1, 1, 2, 2]));

    println!(
        "{}",
        Solution::count_elements_another(vec![1, 1, 3, 3, 5, 5, 7, 7])
    );
    println!("{}", Solution::count_elements_another(vec![1, 2, 3, 3, 4]));
    println!("{}", Solution::count_elements_another(vec![1, 1, 2, 2]));
}
