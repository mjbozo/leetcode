// LeetCode problem 1861: Rotating the Box
// https://leetcode.com/problems/rotating-the-box/description/

fn main() {
    let result = rotate_the_box(vec![vec!['#', '.', '#']]);
    println!("Result = {result:?}");
}

fn rotate_the_box(mut b: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let m = b.len();
    let n = b[0].len();

    for y in 0..m {
        let mut last_open = None;
        for x in (0..n).rev() {
            match b[y][x] {
                '.' if last_open.is_none() => last_open = Some(x),
                '#' if last_open.is_some() => {
                    let o = last_open.unwrap();
                    b[y][o] = '#';
                    b[y][x] = '.';
                    last_open = Some(o - 1);
                }
                '*' => {
                    last_open = None;
                }
                _ => {}
            };
        }
    }

    let mut rotated = vec![vec!['.'; m]; n];
    for y in (0..m).rev() {
        for x in 0..n {
            rotated[x][m - y - 1] = b[y][x];
        }
    }

    return rotated;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = rotate_the_box(vec![vec!['#', '.', '#']]);
        assert_eq!(result, vec![vec!['.'], vec!['#'], vec!['#']]);
    }

    #[test]
    fn example_two() {
        let result = rotate_the_box(vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']]);
        assert_eq!(
            result,
            vec![
                vec!['#', '.'],
                vec!['#', '#'],
                vec!['*', '*'],
                vec!['.', '.']
            ]
        );
    }

    #[test]
    fn example_three() {
        let result = rotate_the_box(vec![
            vec!['#', '#', '*', '.', '*', '.'],
            vec!['#', '#', '#', '*', '.', '.'],
            vec!['#', '#', '#', '.', '#', '.'],
        ]);
        assert_eq!(
            result,
            vec![
                vec!['.', '#', '#'],
                vec!['.', '#', '#'],
                vec!['#', '#', '*'],
                vec!['#', '*', '.'],
                vec!['#', '.', '*'],
                vec!['#', '.', '.']
            ]
        );
    }
}
