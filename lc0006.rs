// LeetCode problem 0006: Zigzag Conversion
// https://leetcode.com/problems/zigzag-conversion/description/

fn main() {
    let result = convert(String::from("PAYPALISHIRING"), 3);
    println!("Result = {result}");
} 

fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s
    }

    let length = s.len() as i32;
    // number of characters in a single 'down and up'
    let block_count = 2 * num_rows - 2;
    // number of 'down and ups' there are
    let iterations = (length / block_count) + 1 + if length % block_count > num_rows { 1 } else { 0 };
    let mut ans = String::with_capacity(length as usize);

    // the motivation here is to loop through each row and index ahead into the correct character of the input string
    // to append to the answer
    for i in 0..num_rows {
        let mut p = 0;
        while p < iterations * block_count {
            // there are diagonals we can find by offsetting from vertical ahead of diagonal
            if i > 0 && p > 0 && p - i < length && i != num_rows - 1 {
                let c = s.as_bytes()[(p - i) as usize] as char;
                ans.push(c);
            }

            // append vertical
            if p + i < length {
                let c = s.as_bytes()[(p + i) as usize] as char;
                ans.push(c);
            }

            // move along to next vertical section
            p += block_count;
        }
    }

    return ans;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = convert(String::from("PAYPALISHIRING"), 3);
        assert_eq!(result, String::from("PAHNAPLSIIGYIR"));
    }

    #[test]
    fn example_two() {
        let result = convert(String::from("PAYPALISHIRING"), 4);
        assert_eq!(result, String::from("PINALSIGYAHRPI"));
    }

    #[test]
    fn example_three() {
        let result = convert(String::from("A"), 1);
        assert_eq!(result, String::from("A"));
    }
}