pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn are_sentences_similar(
        sentence1: Vec<String>,
        sentence2: Vec<String>,
        similar_pairs: Vec<Vec<String>>,
    ) -> bool {
        if sentence1.len() != sentence2.len() {
            return false;
        }

        let pairset: HashSet<(String, String)> = similar_pairs
            .iter()
            .map(|pair| {
                (pair[0].clone(), pair[1].clone())
            })
            .collect();

        sentence1.iter().zip(sentence2.iter()).all(|(w1, w2)| {
            w1 == w2 || pairset.contains(&(w1.clone(), w2.clone())) || pairset.contains(&(w2.clone(), w1.clone()))
        })
    }
}

fn main() {
    let sentence1: Vec<String> = vec!["great", "acting", "skills"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let sentence2 = vec!["fine", "drama", "talent"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let similar_pairs: Vec<Vec<String>> = vec![
        vec!["great", "fine"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        vec!["drama", "acting"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        vec!["skills", "talent"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    ];
    println!(
        "{}",
        Solution::are_sentences_similar(sentence1, sentence2, similar_pairs)
    )
}
