pub struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = nums.clone();
        for i in 1..ans.len() {
            ans[i] += ans[i - 1]
        }
        ans
    }

    pub fn running_sum_scan(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .scan(0, |acc, x| {
                *acc = *acc + *x;
                Some(*acc)
            })
            .collect()
    }
}

fn main() {
    println!("ans: {:?}", Solution::running_sum(vec![1, 2, 3, 4]));
    println!("ans: {:?}", Solution::running_sum_scan(vec![1, 2, 3, 4]));
}
