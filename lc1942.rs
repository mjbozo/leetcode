// LeetCode problem 1942: The Number of the Smallest Unoccupied Chair
// https://leetcode.com/problems/the-number-of-the-smallest-unoccupied-char/description/

fn main() {
    let result = smallest_chair(
        vec![
            vec![33889, 98676],
            vec![80071, 89737],
            vec![44118, 52565],
            vec![52992, 84310],
            vec![78492, 88209],
            vec![21695, 67063],
            vec![84662, 95452],
            vec![98048, 98856],
            vec![98411, 99433],
            vec![55333, 56548],
            vec![65375, 88566],
            vec![55011, 62821],
            vec![48548, 48656],
            vec![87396, 94825],
            vec![55273, 81868],
            vec![75629, 91467],
        ],
        6,
    );
    println!("Result = {result}");
}

fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
    let leave_times = times.clone();
    let mut ordered_leave_times = leave_times.iter().enumerate().collect::<Vec<_>>();
    ordered_leave_times.sort_unstable_by_key(|v| v.1[1]);

    let mut ordered_arrival_times = times.iter().enumerate().collect::<Vec<_>>();
    ordered_arrival_times.sort_unstable_by_key(|v| v.1[0]);

    let mut chairs = vec![false; times.len()];
    let mut friends = vec![None; times.len()];

    for arrive in ordered_arrival_times {
        for depart in &ordered_leave_times {
            if depart.1[1] <= arrive.1[0] {
                // release chairs
                if let Some(i) = friends[depart.0] {
                    chairs[i] = false;
                    friends[depart.0] = None;
                }
            } else {
                break;
            }
        }

        // sit in minimum chair
        for (i, chair) in chairs.iter().enumerate() {
            if *chair == false {
                if arrive.0 as i32 == target_friend {
                    return i as i32;
                }
                chairs[i] = true;
                friends[arrive.0] = Some(i);
                break;
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = smallest_chair(vec![vec![1, 4], vec![2, 3], vec![4, 6]], 1);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_two() {
        let result = smallest_chair(vec![vec![3, 10], vec![1, 5], vec![2, 6]], 0);
        assert_eq!(result, 2);
    }
}
