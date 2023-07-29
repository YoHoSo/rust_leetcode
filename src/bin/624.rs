use std::vec;

pub struct Solution;

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let (mins, maxs) =
            arrays
                .into_iter()
                .fold((Vec::new(), Vec::new()), |(mut mins, mut maxs), array| {
                    mins.push(array[0]);
                    maxs.push(array[array.len() - 1]);
                    (mins, maxs)
                });

        let (min_min_idx, min_min_val) = mins.iter().enumerate().min_by_key(|(_, &x)| x).unwrap();
        let (max_max_idx, max_max_val) = maxs.iter().enumerate().max_by_key(|(_, &x)| x).unwrap();

        if min_min_idx != max_max_idx {
            max_max_val - min_min_val
        } else {
            let min_second_val = mins
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != min_min_idx)
                .min_by_key(|(_, &x)| x)
                .unwrap()
                .1
                .clone();
            let max_second_val = maxs
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != max_max_idx)
                .max_by_key(|(_, &x)| x)
                .unwrap()
                .1
                .clone();
            // let min_second_val = mins.iter().filter(|&&x| x != *min_min_val).min().unwrap_or(min_min_val);
            // let max_second_val = maxs.iter().filter(|&&x| x != *max_max_val).max().unwrap_or(max_max_val);
            println!(
                "{} {} {} {}",
                min_min_val, min_second_val, max_max_val, max_second_val
            );

            if (min_second_val - min_min_val).abs() < (max_max_val - max_second_val).abs() {
                max_max_val - min_second_val
            } else {
                max_second_val - min_min_val
            }
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_distance(vec![vec![1, 2, 3], vec![0, 5], vec![1, 2, 3]])
    );
    println!(
        "{}",
        Solution::max_distance(vec![vec![-1, -1, -1], vec![-2, -2, -1]])
    );
    println!(
        "{}",
        Solution::max_distance(vec![
            vec![-10, -9, -9, -9, -7, -2, -1, 2, 4],
            vec![-9, -7, -6, -6, -3, 0, 1, 3],
            vec![-10, -9, -2, -1, 1, 3]
        ])
    );
}
