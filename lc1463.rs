// LeetCode problem 1463: Cherry Pickup II
// https://leetcode.com/problems/cherry-pickup-ii/description/

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq, PartialEq, Debug)]
struct ScoredCell {
    score: i32,
    point: (i32, i32)
}

impl ScoredCell {
    fn from(score: i32, point: (i32, i32)) -> Self {
        return ScoredCell { score, point };
    }
}

impl PartialOrd for ScoredCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.score.partial_cmp(&other.score);
    }
}

impl Ord for ScoredCell {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.score.cmp(&other.score);
    }
}

fn main() {
    let result = cherry_pickup(vec![
        vec![3, 1, 1],
        vec![2, 5, 1],
        vec![1, 5, 5],
        vec![2, 1, 1],
    ]);
    println!("Result = {result}");
}

fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    let width = grid[0].len();
    let height = grid.len();
    let robot_one_start = (0_i32, 0_i32);
    let robot_two_start = (width as i32 - 1, 0);

    // let robot_one_next = next(robot_one_start, width, height);
    // let robot_two_next = next(robot_two_start, width, height);
    // println!("One Next: {robot_one_next:?}");
    // println!("Two Next: {robot_two_next:?}");

    let robot_one_score_start = grid[robot_one_start.1 as usize][robot_one_start.0 as usize];
    let robot_two_score_start = grid[robot_two_start.1 as usize][robot_two_start.0 as usize];
    
    let mut open_set_one = BinaryHeap::new();
    let mut came_from_one = HashMap::new();
    let mut g_score_one = HashMap::new();
    open_set_one.push(ScoredCell::from(robot_one_score_start, robot_one_start));
    while open_set_one.len() != 0 {
        let current = open_set_one.pop().unwrap();
        let next = next(current.point, width, height);
        if next.is_none() {
            continue;
        }

        let next = next.unwrap();
        for n in next {
            let value = grid[n.1 as usize][n.0 as usize] + current.score;
            let existing = g_score_one.get(&n).unwrap_or(&-1);
            if value > *existing {
                g_score_one.entry(n).and_modify(|v| *v = value).or_insert(value);
                came_from_one.entry(n).and_modify(|v| *v = current.point).or_insert(current.point);
            }
            open_set_one.push(ScoredCell::from(value, n));
        }
    }

    // println!("Scores:\n{g_score_one:?}");
    let end = g_score_one.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    println!("End: {end:?}");

    println!("Path:");
    let mut c = end.0;
    while came_from_one.contains_key(&c) {
        println!("{c:?}");
        c = came_from_one.get(c).unwrap();
    }
    println!("{c:?}");

    // let mut open_set_two = BinaryHeap::new();
    // let mut g_score_two = HashMap::new();
    // open_set_two.push(ScoredCell::from(robot_two_score_start, robot_two_start));
    // while open_set_two.len() != 0 {
        
    // }

    0
}

fn next(pos: (i32, i32), width: usize, height: usize) -> Option<Vec<(i32, i32)>> {
    let x = pos.0;
    let y = pos.1;
    let width = width as i32;
    let height = height as i32;
    
    if y == height - 1 {
        return None;
    }

    let mut n = vec![];
    if x > 0 {
        n.push((x - 1, y + 1));
    }
    n.push((x, y + 1));
    if x < width - 1 {
        n.push((x + 1, y + 1));
    }

    return Some(n);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = cherry_pickup(vec![
            vec![3, 1, 1],
            vec![2, 5, 1],
            vec![1, 5, 5],
            vec![2, 1, 1],
        ]);
        assert_eq!(result, 24);
    }

    #[test]
    fn example_two() {
        let result = cherry_pickup(vec![
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![2, 0, 0, 0, 0, 3, 0],
            vec![2, 0, 9, 0, 0, 0, 0],
            vec![0, 3, 0, 5, 4, 0, 0],
            vec![1, 0, 2, 3, 0, 0, 6],
        ]);
        assert_eq!(result, 28);
    }
}
