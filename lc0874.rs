// LeetCode problem 0874: Walking Robot Simulation
// https://leetcode.com/problems/walking-robot-simulation/description/

use std::collections::HashSet;

fn main() {
    let result = robot_sim(vec![4, -1, 3], vec![]);
    println!("Result = {result}");
}

enum Heading {
    N,
    S,
    E,
    W,
}

impl Heading {
    fn rotate_cw(&self) -> Self {
        return match self {
            Heading::N => Heading::E,
            Heading::E => Heading::S,
            Heading::S => Heading::W,
            Heading::W => Heading::N,
        };
    }

    fn rotate_ccw(&self) -> Self {
        return match self {
            Heading::N => Heading::W,
            Heading::E => Heading::N,
            Heading::S => Heading::E,
            Heading::W => Heading::S,
        };
    }
}

fn obstacle_at(obstacles: &HashSet<Vec<i32>>, x: i32, y: i32) -> bool {
    let check_pos = vec![x, y];
    return obstacles.contains(&check_pos);
}

fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    let mut obstacle_hash: HashSet<Vec<i32>> = HashSet::new();
    for obstacle in obstacles {
        obstacle_hash.insert(obstacle);
    }

    let mut position = (0, 0, Heading::N);
    let mut furthest = 0;

    for command in commands {
        match command {
            -2 => position.2 = position.2.rotate_ccw(),
            -1 => position.2 = position.2.rotate_cw(),
            x if (1..=9).contains(&x) => {
                match position.2 {
                    Heading::N => {
                        for _ in 0..x {
                            if !obstacle_at(&obstacle_hash, position.0, position.1 + 1) {
                                position = (position.0, position.1 + 1, position.2);
                            }
                        }
                    }
                    Heading::E => {
                        for _ in 0..x {
                            if !obstacle_at(&obstacle_hash, position.0 + 1, position.1) {
                                position = (position.0 + 1, position.1, position.2);
                            }
                        }
                    }
                    Heading::S => {
                        for _ in 0..x {
                            if !obstacle_at(&obstacle_hash, position.0, position.1 - 1) {
                                position = (position.0, position.1 - 1, position.2);
                            }
                        }
                    }
                    Heading::W => {
                        for _ in 0..x {
                            if !obstacle_at(&obstacle_hash, position.0 - 1, position.1) {
                                position = (position.0 - 1, position.1, position.2);
                            }
                        }
                    }
                };
            }
            _ => panic!("poo"),
        };

        let distance_sq = (position.0 * position.0) + (position.1 * position.1);
        if distance_sq > furthest {
            furthest = distance_sq;
        }
    }

    return furthest;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = robot_sim(vec![4, -1, 3], vec![]);
        assert_eq!(result, 25);
    }

    #[test]
    fn example_two() {
        let result = robot_sim(vec![4, -1, 4, -2, 4], vec![vec![2, 4]]);
        assert_eq!(result, 65);
    }

    #[test]
    fn example_three() {
        let result = robot_sim(vec![6, -1, -1, 6], vec![vec![0, 0]]);
        assert_eq!(result, 36);
    }
}
