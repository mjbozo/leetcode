// LeetCode problem 1079: Letter Tile Possibilities
// https://leetcode.com/problems/letter-tile-possibilities/description/

use std::collections::{HashMap, HashSet};

fn main() {
    let result = num_tile_possibilities(String::from("BANANA"));
    println!("Result = {result}");
}

fn num_tile_possibilities(tiles: String) -> i32 {
    let mut opts = HashSet::new();
    let mut tiles = tiles;
    let mut tile_chars = tiles.chars().collect::<Vec<char>>();
    tile_chars.sort_unstable();
    tiles = tile_chars.into_iter().collect::<String>();
    try_all_options(String::new(), &tiles, &mut opts);

    let mut total = 0;
    for opt in opts {
        let mut freq = HashMap::new();
        for b in opt.bytes() {
            freq.entry(b).and_modify(|v| *v += 1).or_insert(1);
        }

        let all_combinations: i32 = (1..=opt.len()).product::<usize>() as i32;
        let mut duplicate_reduction = 1;

        for (_, v) in freq {
            duplicate_reduction *= (1..=v).product::<usize>() as i32;
        }

        total += all_combinations / duplicate_reduction;
    }

    return total;
}

fn try_all_options(mut current: String, tiles: &str, mut opts: &mut HashSet<String>) {
    if tiles.len() == 0 {
        if current.len() != 0 {
            opts.insert(current);
        }
        return;
    }

    for i in 0..tiles.len() {
        try_all_options(current.clone(), &tiles[i + 1..], &mut opts);
        current += &tiles[i..=i];
        try_all_options(current.clone(), &tiles[i + 1..], &mut opts);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = num_tile_possibilities(String::from("AAB"));
        assert_eq!(result, 8);
    }

    #[test]
    fn example_two() {
        let result = num_tile_possibilities(String::from("AAABBC"));
        assert_eq!(result, 188);
    }

    #[test]
    fn example_three() {
        let result = num_tile_possibilities(String::from("V"));
        assert_eq!(result, 1);
    }
}
