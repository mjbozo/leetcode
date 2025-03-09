// LeetCode problem 2379: Minimum Recolors to Get K Consecutive Black Blocks
// https://leetcode.com/problems/minimum-recolors-to-get-k-consecutive-black-blocks/description/

fn main() {
    let result = minimum_recolors(String::from("WBBWWBBWBW"), 7);
    println!("Result = {result}");
}

fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let n = blocks.len() as i32;
    let b = blocks.as_bytes();
    let mut whites = 0;
    for i in 0..k {
        match b[i as usize] {
            b'W' => whites += 1,
            _ => {}
        }
    }

    let mut left = 0;
    let mut right = k;
    let mut recolors = whites;

    while right < n {
        if b[left as usize] == b'W' {
            whites -= 1;
        }
        if b[right as usize] == b'W' {
            whites += 1;
        }

        recolors = recolors.min(whites);
        left += 1;
        right += 1;
    }

    return recolors;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = minimum_recolors(String::from("WBBWWBBWBW"), 7);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = minimum_recolors(String::from("WBWBBBW"), 2);
        assert_eq!(result, 0);
    }
}
