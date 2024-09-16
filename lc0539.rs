// LeetCode problem 0539: Minimum Time Difference
// https://leetcode.com/problems/minimum-time-difference/description/

fn main() {
    let result = find_min_difference(vec!["23:59".into(), "00:00".into()]);
    println!("Result = {result}");
}

fn find_min_difference(time_points: Vec<String>) -> i32 {
    // convert time_points into a list of minutes, and sort
    let mut minute_points = time_points
        .into_iter()
        .map(|p| {
            let mut split = p.split(":");
            let hours = split.next().unwrap().parse::<i32>().unwrap() * 60;
            let minutes = split.next().unwrap().parse::<i32>().unwrap();
            hours + minutes
        })
        .collect::<Vec<_>>();
    minute_points.sort_unstable();

    let minutes_in_12_hours = 720;
    let minutes_in_24_hours = 1440;
    let mut minimum_diff = minutes_in_24_hours;

    // compare every 2 pairs of neighbouring, sorted elements to check time diff
    for w in minute_points.windows(2) {
        let mut abs_diff = w[1] - w[0];
        if abs_diff > minutes_in_12_hours {
            abs_diff = w[0] + minutes_in_24_hours - w[1];
        }

        if abs_diff < minimum_diff {
            minimum_diff = abs_diff;
        }
    }

    // check array bounds for wrap-around edge case
    let mut wrap_diff = minute_points.last().unwrap() - minute_points.first().unwrap();
    if wrap_diff > minutes_in_12_hours {
        wrap_diff =
            minute_points.first().unwrap() + minutes_in_24_hours - minute_points.last().unwrap();
    }

    return std::cmp::min(wrap_diff, minimum_diff);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_min_difference(vec!["23:59".into(), "00:00".into()]);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_two() {
        let result = find_min_difference(vec!["00:00".into(), "23:59".into(), "00:00".into()]);
        assert_eq!(result, 0);
    }
}
