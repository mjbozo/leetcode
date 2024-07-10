// LeetCode problem 0787: Cheapest Flights Within K Stops
// https://leetcode.com/problems/cheapest-flights-within-k-stops/description/

#[derive(Debug)]
struct Node {
    edges: Vec<(i32, i32)> // id, cost
}

impl Node {
    fn new() -> Self {
        Node { 
            edges: Vec::new()
        }
    }
}

fn main() {
    let result = find_cheapest_price(4, vec![vec![0, 1, 100], vec![1, 2, 100],
        vec![2, 0, 100], vec![1, 3, 600], vec![2, 3, 200]], 0, 3, 1);
    println!("Result = {result}");
}

fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let mut nodes = vec![];
    for _ in 0..n {
        nodes.push(Node::new());
    }

    for flight in flights {
        let from = flight[0];
        let to = flight[1];
        let cost = flight[2];

        nodes[from as usize].edges.push((to, cost));
    }


    // pathfinding: find cheapest path from src to dst without exceeding k stops
    let cheapest = dfs(&nodes, src, dst, 0, 0, k);

    println!("{nodes:?}");
    return cheapest;
}

fn dfs(nodes: &Vec<Node>, current: i32, goal: i32, score: i32, depth: i32, k: i32) -> i32 {
    if depth > k + 1 {
        return -1;
    }

    if current == goal {
        return score;
    }

    let mut best = -1;
    for edge in nodes[current as usize].edges.iter() {
        let result = dfs(nodes, edge.0, goal, score + edge.1, depth + 1, k);
        if result == -1 {
            continue;
        }
        
        if best == -1 || result < best {
            best = result;
        }
    }

    return best;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_cheapest_price(4, vec![vec![0, 1, 100], vec![1, 2, 100],
            vec![2, 0, 100], vec![1, 3, 600], vec![2, 3, 200]], 0, 3, 1);
        assert_eq!(result, 700);
    }

    #[test]
    fn example_two() {
        let result = find_cheapest_price(3, vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]], 0, 2, 1);
        assert_eq!(result, 200);
    }

    #[test]
    fn example_three() {
        let result = find_cheapest_price(3, vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]], 0, 2, 0);
        assert_eq!(result, 500);
    }
}
