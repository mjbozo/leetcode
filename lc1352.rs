// LeetCode problem 1352: Product of the Last K Numbers
// https://leetcode.com/problems/product-of-the-last-k-numbers/description/

struct ProductOfNumbers {
    current: i32,
    prefix: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        return ProductOfNumbers {
            current: 1,
            prefix: vec![],
        };
    }

    fn add(&mut self, num: i32) {
        if self.current == 0 {
            self.current = num;
            self.prefix = vec![1];
        } else {
            self.prefix.push(self.current);
            self.current *= num;
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        if k > self.prefix.len() {
            return 0;
        }

        return self.current / self.prefix[self.prefix.len() - k];
    }
}

fn main() {
    let mut pn = ProductOfNumbers::new();
    pn.add(3);
    pn.add(0);
    pn.add(2);
    pn.add(5);
    pn.add(4);
    println!("{}", pn.get_product(2));
    println!("{}", pn.get_product(3));
    println!("{}", pn.get_product(4));
    pn.add(8);
    println!("{}", pn.get_product(2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let mut pn = ProductOfNumbers::new();
        pn.add(3);
        pn.add(0);
        pn.add(2);
        pn.add(5);
        pn.add(4);
        assert_eq!(pn.get_product(2), 20);
        assert_eq!(pn.get_product(3), 40);
        assert_eq!(pn.get_product(4), 0);
        pn.add(8);
        assert_eq!(pn.get_product(2), 32);
    }
}
