use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn get_digits(n: i32) -> Vec<i32> {
        let mut digits: Vec<i32> = Vec::new();
        let mut n = n;
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits
    }

    pub fn is_happy(n: i32) -> bool {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut n = n;
        loop {
            let digits = Solution::get_digits(n);
            let sum: i32 = digits.into_iter().map(|x| x * x).sum();

            if sum == 1 {
                return true;
            } else if seen.contains(&sum) {
                return false;
            } else {
                seen.insert(n);
                n = sum;
            }
        }
    }

    pub fn is_happy_elegant(n: i32) -> bool {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut n = n;

        while n != 1 && !seen.contains(&n) {
            seen.insert(n);
            let digits = Solution::get_digits(n);
            n = digits.into_iter().map(|x| x.pow(2)).sum();
        }

        n == 1
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_202() {
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(2), false);

        assert_eq!(Solution::is_happy_elegant(19), true);
        assert_eq!(Solution::is_happy_elegant(2), false);
    }

    #[test]
    fn test_get_digits() {
        assert_eq!(Solution::get_digits(19), vec![9, 1]);
        assert_eq!(Solution::get_digits(2), vec![2]);
    }
}
