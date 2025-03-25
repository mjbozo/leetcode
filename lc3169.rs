// LeetCode problem 3169: Count Days Without Meetings
// https://leetcode.com/problems/count-days-without-meetings/description/

fn main() {
    let result = count_days(1000000000, vec![vec![1, 1000000000]]);
    println!("Result = {result}");
}

fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    let mut latest_end = 0;

    let mut meetings = meetings;
    meetings.sort_unstable_by_key(|v| v[0]);

    for meeting in &meetings {
        let start = meeting[0];
        let end = meeting[1];
        if start > latest_end + 1 {
            result += start - latest_end - 1;
        }
        latest_end = latest_end.max(end);
    }

    result += days - latest_end;
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = count_days(10, vec![vec![5, 7], vec![1, 3], vec![9, 10]]);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = count_days(5, vec![vec![2, 4], vec![1, 3]]);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_three() {
        let result = count_days(6, vec![vec![1, 6]]);
        assert_eq!(result, 0);
    }
}
