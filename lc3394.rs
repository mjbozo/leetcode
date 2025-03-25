// LeetCode problem 3394: Check if Grid can be Cut into Sections
// https://leetcode.com/problems/check-if-grid-can-be-cut-into-sections/description/

fn main() {
    let result = check_valid_cuts(
        5,
        vec![
            vec![1, 0, 5, 2],
            vec![0, 2, 2, 4],
            vec![3, 2, 5, 3],
            vec![0, 4, 4, 5],
        ],
    );
    println!("Result = {result}");
}

fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
    let mut rectangles = rectangles;
    rectangles.sort_unstable_by_key(|v| v[0]);
    if can_cut_vertical(&rectangles) {
        return true;
    }

    rectangles.sort_unstable_by_key(|v| v[1]);
    return can_cut_horizontal(&rectangles);
}

fn can_cut_vertical(recs: &Vec<Vec<i32>>) -> bool {
    let mut latest_end = 0;
    let mut cuts = 0;
    for rec in recs {
        let start = rec[0];
        let end = rec[2];
        if latest_end > 0 && start >= latest_end {
            cuts += 1;
        }

        latest_end = latest_end.max(end);
    }

    return cuts >= 2;
}

fn can_cut_horizontal(recs: &Vec<Vec<i32>>) -> bool {
    let mut latest_end = 0;
    let mut cuts = 0;
    for rec in recs {
        let start = rec[1];
        let end = rec[3];
        if latest_end > 0 && start >= latest_end {
            cuts += 1;
        }

        latest_end = latest_end.max(end);
    }

    return cuts >= 2;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = check_valid_cuts(
            5,
            vec![
                vec![1, 0, 5, 2],
                vec![0, 2, 2, 4],
                vec![3, 2, 5, 3],
                vec![0, 4, 4, 5],
            ],
        );
        assert!(result);
    }

    #[test]
    fn example_two() {
        let result = check_valid_cuts(
            4,
            vec![
                vec![0, 0, 1, 1],
                vec![2, 0, 3, 4],
                vec![0, 2, 2, 3],
                vec![3, 0, 3, 4],
            ],
        );
        assert!(result);
    }

    #[test]
    fn example_three() {
        let result = check_valid_cuts(
            4,
            vec![
                vec![0, 2, 2, 4],
                vec![1, 0, 3, 2],
                vec![2, 2, 3, 4],
                vec![3, 0, 4, 2],
                vec![3, 2, 4, 4],
            ],
        );
        assert!(!result);
    }
}
