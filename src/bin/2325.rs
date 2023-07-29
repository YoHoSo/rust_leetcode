use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut letter_map: HashMap<char, char> = std::collections::HashMap::new();
        let mut i = 0;
        key.replace(" ", "").chars().for_each(|c| {
            letter_map.entry(c).or_insert_with(|| {
                let new_char = ('a' as u8 + i as u8) as char;
                i += 1;
                new_char
            });
        });

        let result = message
            .chars()
            .map(|c| letter_map.get(&c).copied().unwrap_or(c))
            .collect();

        result
    }
}

fn main() {
    let key: String = "the quick brown fox jumps over the lazy dog".into();
    // let key: String = "the".into();
    let message = "vkbs bs t suepuv".into();
    let result = Solution::decode_message(key, message);
    println!("{}", result);
}
