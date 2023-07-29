/*
给你一个正整数 n 。n 中的每一位数字都会按下述规则分配一个符号：

最高有效位 上的数字分配到 正 号。
剩余每位上数字的符号都与其相邻数字相反。
返回所有数字及其对应符号的和。
*/

pub struct Solution;

impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let digits = Solution::get_digits(n);
        let mut should_add = true;
        let sum = digits
            .into_iter()
            .rev()
            .map(|n| {
                let ans = if should_add { n } else { -n };
                should_add = !should_add;
                ans
            })
            .sum();
        sum
    }

    fn get_digits(mut n: i32) -> Vec<i32> {
        let mut digits = Vec::new();
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits
    }
}

fn main() {
    for num in [521, 111, 886996, 10] {
        println!("num is {}", Solution::alternate_digit_sum(num));
    }
}
