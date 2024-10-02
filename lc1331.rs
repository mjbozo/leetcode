// LeetCode problem 1331: Rank Transform of an Array
// https://leetcode.com/problems/rank-transform-of-an-array/description/

fn main() {
    let result = array_rank_transform(vec![40, 10, 20, 30]);
    println!("Result = {result:?}");
}

fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() == 0 {
        return vec![];
    }

    let mut arr = arr.iter().enumerate().collect::<Vec<_>>();
    arr.sort_unstable_by(|(_, x), (_, y)| x.cmp(&y));

    let mut ranked_arr = vec![0; arr.len()];
    let mut rank = 1;
    let mut prev = arr[0].1;

    for x in arr {
        if x.1 != prev {
            rank += 1;
        }

        ranked_arr[x.0] = rank;
        prev = x.1;
    }

    return ranked_arr;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = array_rank_transform(vec![40, 10, 20, 30]);
        assert_eq!(result, vec![4, 1, 2, 3]);
    }

    #[test]
    fn example_two() {
        let result = array_rank_transform(vec![100, 100, 100]);
        assert_eq!(result, vec![1, 1, 1]);
    }

    #[test]
    fn example_three() {
        let result = array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]);
        assert_eq!(result, vec![5, 3, 4, 2, 8, 6, 7, 1, 3]);
    }
}
