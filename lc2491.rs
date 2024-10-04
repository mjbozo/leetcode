// LeetCode problem 2419: Divide Players Into Teams of Equal Skill
// https://leetcode.com/problems/divide-players-into-teams-of-equal-skill/description/

fn main() {
    let result = divide_players(vec![3, 2, 5, 1, 3, 4]);
    println!("Result = {result:?}");
}

fn divide_players(skill: Vec<i32>) -> i64 {
    let mut total = 0;
    for player in &skill {
        total += *player;
    }

    let n = skill.len();
    let target = (total as f64 * 2.0) / n as f64;
    if target.fract() != 0.0 {
        return -1;
    }

    let mut skill = skill;
    skill.sort_unstable();

    let mut chemistry: i64 = 0;

    for i in 0..n / 2 {
        if skill[i] + skill[n - i - 1] != target as i32 {
            return -1;
        }
        chemistry += (skill[i] * skill[n - i - 1]) as i64;
    }

    return chemistry;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = divide_players(vec![3, 2, 5, 1, 3, 4]);
        assert_eq!(result, 22);
    }

    #[test]
    fn example_two() {
        let result = divide_players(vec![3, 4]);
        assert_eq!(result, 12);
    }

    #[test]
    fn example_three() {
        let result = divide_players(vec![1, 1, 2, 3]);
        assert_eq!(result, -1);
    }
}
