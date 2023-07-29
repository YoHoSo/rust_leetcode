pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn calculate_time(keyboard: String, word: String) -> i32 {
        let mut letter_map: HashMap<char, u8> = HashMap::new();
        keyboard.chars().into_iter().enumerate().for_each(|(i, c)| {
            letter_map.insert(c, i as u8);
        });

        let mut current = 0;
        word.chars()
            .into_iter()
            .map(|c| {
                let dist = (*letter_map.get(&c).unwrap() as i32 - current).abs();
                current = *letter_map.get(&c).unwrap() as i32;
                dist
            })
            .sum::<i32>()
    }
}

fn main() {
    println!(
        "{}",
        Solution::calculate_time("abcdefghijklmnopqrstuvwxyz".to_string(), "cba".to_string())
    );
    println!(
        "{}",
        Solution::calculate_time(
            "pqrstuvwxyzabcdefghijklmno".to_string(),
            "leetcode".to_string()
        )
    );
}
