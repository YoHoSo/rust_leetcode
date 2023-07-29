pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn largest_unique_number(nums: Vec<i32>) -> i32 {
        let mut num_set: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            *num_set.entry(n).or_insert(0) += 1;
        }

        let mut ans = -1;
        num_set.iter().for_each(|(&k, &v)| {
            if v == 1 && k > ans {
                ans = k;
            }
        });
        ans
    }

    pub fn largest_unique_number_less_memory(nums: Vec<i32>) -> i32 {}
}

// class Solution:
//     def largestUniqueNumber(self, nums: List[int]) -> int:
//         return max((key  for key, val in Counter(nums).items() if val == 1), default=-1)

// 作者：int_64
// 链接：https://leetcode.cn/problems/largest-unique-number/solutions/1862934/by-100000000000001-abyf/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

fn main() {
    println!("{}", Solution::largest_unique_number(vec![9, 9, 8, 8]));
    println!(
        "{}",
        Solution::largest_unique_number(vec![5, 7, 3, 9, 4, 9, 8, 3, 1])
    );
}
