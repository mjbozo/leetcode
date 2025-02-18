// LeetCode problem 2375: Construct Smallest Number From DI String
// https://leetcode.com/problems/construct-smallest-number-from-di-string/description/

fn main() {
    let result = smallest_number(String::from("IIIDIDDD"));
    println!("Result = {result}");
}

fn smallest_number(pattern: String) -> String {
    let n = pattern.len();
    let bytes = pattern.as_bytes();
    let mut nums = vec![0; n + 1];
    nums[0] = 1;

    let mut next_min = 2;

    let mut i = 0;
    while i < n {
        match bytes[i] {
            b'I' => {
                nums[i + 1] = next_min;
                next_min += 1;
            }
            b'D' => {
                let mut j = i;
                while bytes[j] == b'D' {
                    nums[j] += 1;
                    if j == 0 {
                        break;
                    }
                    j -= 1;
                }
                nums[i + 1] = nums[i] - 1;
                next_min += 1;
            }
            _ => {}
        }
        i += 1;
    }

    return nums.into_iter().map(|v| v.to_string()).collect();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = smallest_number(String::from("IIIDIDDD"));
        assert_eq!(result, String::from("123549876"));
    }

    #[test]
    fn example_two() {
        let result = smallest_number(String::from("DDD"));
        assert_eq!(result, String::from("4321"));
    }
}
