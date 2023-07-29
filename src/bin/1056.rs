pub struct Solution;

impl Solution {
    pub fn confusing_number(n: i32) -> bool {
        let digits = Solution::get_digits(n);
        let mut cnt = digits.len();
        for (digit, reverse_d) in digits.iter().zip(digits.iter().rev()) {
            if let Some(rotated_digit) = Solution::get_rotation(*digit) {
                if rotated_digit == *reverse_d {
                    cnt -= 1;
                }
            } else {
                return false;
            }
        }

        cnt > 0
    }

    fn get_digits(n: i32) -> Vec<i32> {
        let mut digits: Vec<i32> = Vec::new();
        let mut n = n;
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits
    }

    fn get_rotation(n: i32) -> Option<i32> {
        assert!(n < 10);
        let res = match n {
            0 => 0,
            1 => 1,
            6 => 9,
            8 => 8,
            9 => 6,
            _ => return None,
        };
        Some(res)
    }
}

fn main() {
    for k in [25, 11, 89, 6, 916] {
        println!("{}", Solution::confusing_number(k));
    }
}
