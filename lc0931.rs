// LeetCode problem 0931: Minimum Falling Path Sum
// https://leetcode.com/problems/minimum-falling-path-sum/description/

fn main() {
    let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
    let result = min_falling_path_sum(matrix);
    println!("Result = {result}");
}

fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let mut records: Vec<i32> = vec![];
    let starting_row = &matrix[0];
    for &cell in starting_row.iter() {
        records.push(cell);
    }
    let mut next_records = records.clone();

    for row in &matrix[1..] {
        for (index, cell) in row.iter().enumerate() {
            let paths = get_paths(index, &records);
            let min_path = paths.iter().min().unwrap_or(&0);
            next_records[index] = *min_path + cell;
        }
        records = next_records.clone();
    }

    return *records.iter().min().unwrap_or(&0);
}

fn get_paths(i: usize, records: &Vec<i32>) -> Vec<i32> {
    let mut paths = vec![];
    if i != 0 {
        paths.push(records[i - 1]);
    }
    paths.push(records[i]);
    if i < records.len() - 1 {
        paths.push(records[i + 1]);
    }
    return paths;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
        let result = min_falling_path_sum(matrix);
        assert_eq!(result, 13);
    }

    #[test]
    fn example_two() {
        let matrix = vec![vec![-19, 57], vec![-40, -5]];
        let result = min_falling_path_sum(matrix);
        assert_eq!(result, -59);
    }
}
