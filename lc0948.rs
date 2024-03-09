// LeetCode problem 0948: Bag of Tokens
// https://leetcode.com/problems/bag-of-tokens/description/

fn main() {
    let result = bag_of_tokens_score(vec![100], 50);
    println!("Result = {result}");
}

fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
    let mut tokens = &mut tokens[..];
    tokens.sort_unstable();

    let mut max_score = 0;
    let mut current_score = 0;

    while tokens.len() > 0 {
        if tokens[0] <= power {
            current_score += 1;
            power -= tokens[0];
            if current_score > max_score {
                max_score = current_score;
            }
            tokens = &mut tokens[1..];
        } else if current_score > 0 {
            current_score -= 1;
            power += tokens.last().unwrap_or(&0);
            let len = tokens.len();
            tokens = &mut tokens[..len-1];
        } else {
            break;
        }
    }

    return max_score;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = bag_of_tokens_score(vec![100], 50);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_two() {
        let result = bag_of_tokens_score(vec![200,100], 150);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_three() {
        let result = bag_of_tokens_score(vec![100,200,300,400], 200);
        assert_eq!(result, 2);
    }
}
