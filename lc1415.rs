// LeetCode problem 1415: The k-th Lexicographical String of All Happy Strings of Length n
// https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/description/

fn main() {
    let result = get_happy_string(1, 3);
    println!("Result = {result}");
}

fn get_happy_string(n: i32, k: i32) -> String {
    let mut strings = vec![];
    build_string(String::new(), n, k, &mut strings);
    if strings.len() < k as usize {
        return String::new();
    }

    strings.sort_unstable();
    return strings[k as usize - 1].clone();
}

fn build_string(current: String, n: i32, k: i32, mut strings: &mut Vec<String>) {
    if strings.len() > k as usize {
        return;
    }

    if current.len() == n as usize {
        strings.push(current);
        return;
    }

    let bytes = current.as_bytes();

    if current.len() == 0 || bytes[bytes.len() - 1] != b'a' {
        build_string(current.clone() + "a", n, k, &mut strings);
    }

    if current.len() == 0 || bytes[bytes.len() - 1] != b'b' {
        build_string(current.clone() + "b", n, k, &mut strings);
    }

    if current.len() == 0 || bytes[bytes.len() - 1] != b'c' {
        build_string(current.clone() + "c", n, k, &mut strings);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = get_happy_string(1, 3);
        assert_eq!(result, String::from("c"));
    }

    #[test]
    fn example_two() {
        let result = get_happy_string(1, 4);
        assert_eq!(result, String::from(""));
    }

    #[test]
    fn example_three() {
        let result = get_happy_string(3, 9);
        assert_eq!(result, String::from("cab"));
    }
}
