pub struct Solution;
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let max_n = candy_type.len() / 2;
        let mut seen: HashSet<i32> = HashSet::new();

        candy_type.into_iter().any(|each_candy_type| {
            if !seen.contains(&each_candy_type) {
                seen.insert(each_candy_type);
            }
            if seen.len() == max_n {
                return true;
            }
            false
        });
        seen.len() as i32
    }

    pub fn distribute_candies_elegant(candy_type: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let unique_n = candy_type.iter().collect::<HashSet<&i32>>().len();
        unique_n.min(candy_type.len() / 2) as i32
    }
}

fn main() {}

mod test {
    use super::*;

    #[test]
    fn test_575() {
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 3]), 2);
        assert_eq!(Solution::distribute_candies(vec![6, 6, 6, 6]), 1);
    }
}
