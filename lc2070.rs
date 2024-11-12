// LeetCode problem 2070: Most Beautiful Item for Each Query
// https://leetcode.com/problems/most-beautiful-item-for-each-query/description/

fn main() {
    let result = maximum_beauty(
        vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]],
        vec![1, 2, 3, 4, 5, 6],
    );
    println!("Result = {result:?}");
}

fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut items = items;
    items.sort_unstable_by(|a, b| {
        if a[0] != b[0] {
            return a[0].cmp(&b[0]);
        }
        return b[1].cmp(&a[1]);
    });

    let mut trends = vec![];
    trends.push(&items[0]);

    for i in 1..items.len() {
        let last = trends.last().unwrap();
        if items[i][0] != last[0] && last[1] < items[i][1] {
            trends.push(&items[i]);
        }
    }

    let mut beauties = Vec::with_capacity(queries.len());
    for query in queries {
        match trends.binary_search_by_key(&query, |v| v[0]) {
            Ok(x) => {
                beauties.push(trends[x][1]);
            }
            Err(x) => {
                if x == 0 {
                    beauties.push(0);
                } else {
                    beauties.push(trends[x - 1][1]);
                }
            }
        }
    }

    return beauties;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = maximum_beauty(
            vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]],
            vec![1, 2, 3, 4, 5, 6],
        );
        assert_eq!(result, vec![2, 4, 5, 5, 6, 6]);
    }

    #[test]
    fn example_two() {
        let result = maximum_beauty(
            vec![vec![1, 2], vec![1, 2], vec![1, 3], vec![1, 4]],
            vec![1],
        );
        assert_eq!(result, vec![4]);
    }

    #[test]
    fn example_three() {
        let result = maximum_beauty(vec![vec![10, 1000]], vec![5]);
        assert_eq!(result, vec![0]);
    }
}
