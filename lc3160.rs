// LeetCode problem 3160: Find the Number of Distinct Colors Among the Balls
// https://leetcode.com/problems/find-the-number-of-distinct-colors-among-the-balls/description/

use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn main() {
    let result = query_results(4, vec![vec![1, 4], vec![2, 5], vec![1, 3], vec![3, 4]]);
    println!("Result = {:?}", result);
}

fn query_results(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut colour_freq = HashMap::new();
    let mut distinct = Vec::with_capacity(queries.len());
    let mut ball_colours = HashMap::new();

    for q in queries {
        let colour = q[1];
        let ball_colour = *ball_colours.entry(q[0]).or_default();

        if ball_colour != 0 {
            let e = colour_freq.entry(ball_colour).and_modify(|v| *v -= 1);
            if let Entry::Occupied(s) = e {
                if *s.get() == 0 {
                    s.remove();
                }
            }
        }

        ball_colours
            .entry(q[0])
            .and_modify(|v| *v = q[1])
            .or_insert(q[1]);
        colour_freq
            .entry(colour)
            .and_modify(|v| *v += 1)
            .or_insert(1);
        distinct.push(colour_freq.len() as i32);
    }

    return distinct;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = query_results(4, vec![vec![1, 4], vec![2, 5], vec![1, 3], vec![3, 4]]);
        assert_eq!(result, vec![1, 2, 2, 3]);
    }

    #[test]
    fn example_two() {
        let result = query_results(
            4,
            vec![vec![0, 1], vec![1, 2], vec![2, 2], vec![3, 4], vec![4, 5]],
        );
        assert_eq!(result, vec![1, 2, 2, 3, 4]);
    }
}
