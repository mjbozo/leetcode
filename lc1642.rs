// LeetCode problem 1642: Furthest Building You Can Reach
// https://leetcode.com/problems/furthest-building-you-can-reach/description/

fn main() {
    let result = furthest_building(vec![4, 2, 7, 6, 9, 14, 12], 5, 1);
    println!("Result = {result}");
}

fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
    let len = heights.len();
    if len == 1 {
        return 0;
    }

    let mut ladder_jumps: Vec<i32> = vec![0; ladders as usize];
    let mut brick_sum = 0;
    for i in 1..len {
        let height_diff = heights[i] - heights[i - 1];
        if height_diff <= 0 {
            continue;
        }

        let ladders_used = ladder_jumps.len();
        if (ladders_used as i32) < ladders {
            for j in 0..ladders_used {
                if height_diff > ladder_jumps[j] {
                    ladder_jumps.insert(j, height_diff);
                    break;
                }
            }
        } else {
            let lowest_ladder = ladder_jumps.last();
            if lowest_ladder.is_some() && height_diff > *lowest_ladder.unwrap() {
                for j in 0..ladder_jumps.len() {
                    if height_diff > ladder_jumps[j] {
                        ladder_jumps.insert(j, height_diff);
                        break;
                    }
                }
                let removal = ladder_jumps.pop().unwrap_or(0);
                brick_sum += removal;
            } else {
                brick_sum += height_diff;
            }
        }

        if brick_sum > bricks {
            return i as i32 - 1;
        }
    }

    return len as i32 - 1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = furthest_building(vec![4, 2, 7, 6, 9, 14, 12], 5, 1);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_two() {
        let result = furthest_building(vec![4, 12, 2, 7, 3, 18, 20, 3, 19], 10, 2);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_three() {
        let result = furthest_building(vec![14, 3, 19, 3], 17, 0);
        assert_eq!(result, 3);
    }
}
