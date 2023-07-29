pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut slow = 0;
        for fast in 1..nums.len() {
            if nums[slow] != nums[fast] {
                slow += 1;
                nums[slow] = nums[fast];
            }
        }
        (slow + 1) as i32
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::remove_duplicates(vec![0, 0, 1, 1, 2, 3, 3, 4].as_mut())
    );
}
