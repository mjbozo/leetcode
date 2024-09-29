// LeetCode problem 0432: All O'one Data Structure
// https://leetcode.com/problems/all-oone-data-structure/description/

use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
struct AllOne {
    word_counts: HashMap<String, i32>,
    count_words: BTreeMap<i32, Vec<String>>,
    max: (String, i32),
    min: (String, i32),
}

impl AllOne {
    fn new() -> Self {
        return AllOne {
            word_counts: HashMap::new(),
            count_words: BTreeMap::new(),
            max: (String::new(), 0),
            min: (String::new(), 0),
        };
    }

    fn inc(&mut self, key: String) {
        if let Some(e) = self.word_counts.get_mut(&key) {
            // entry already exists
            let words = self.count_words.get_mut(e).unwrap();
            words.retain(|x| *x != key);
            if words.len() == 0 {
                self.count_words.remove(e);
            }

            *e += 1;
            self.count_words
                .entry(*e)
                .and_modify(|v| v.push(key.clone()))
                .or_insert(vec![key.clone()]);

            if *e > self.max.1 {
                self.max = (key.clone(), *e);
            }

            if self.min.0 == key {
                if let Some(m) = self.count_words.get(&self.min.1) {
                    // another key has current min count
                    self.min.0 = m.first().unwrap().clone();
                } else {
                    self.min.1 += 1;
                }
            }
        } else {
            // new entry
            self.min = (key.clone(), 1);
            if self.max.1 == 0 {
                self.max = (key.clone(), 1);
            }

            self.word_counts.insert(key.clone(), 1);

            self.count_words
                .entry(1)
                .and_modify(|v| v.push(key.clone()))
                .or_insert(vec![key.clone()]);
        }
    }

    fn dec(&mut self, key: String) {
        // we can assume key exists
        let entry = self.word_counts.get_mut(&key).unwrap();
        if *entry == 1 {
            self.word_counts.remove(&key);

            let words = self.count_words.get_mut(&1).unwrap();
            words.retain(|x| *x != key);
            if words.len() == 0 {
                self.count_words.remove(&1);
            }

            if self.min.0 == key {
                if let Some((k, v)) = self.count_words.first_key_value() {
                    self.min = (v.first().unwrap().clone(), *k);
                } else {
                    self.min = (String::new(), 0);
                }
            }

            if self.max.0 == key {
                if let Some((k, v)) = self.count_words.last_key_value() {
                    self.max = (v.first().unwrap().clone(), *k);
                } else {
                    self.max = (String::new(), 0);
                }
            }
        } else {
            let words = self.count_words.get_mut(entry).unwrap();
            words.retain(|x| *x != key);
            if words.len() == 0 {
                self.count_words.remove(entry);
            }

            *entry -= 1;
            self.count_words
                .entry(*entry)
                .and_modify(|v| v.push(key.clone()))
                .or_insert(vec![key.clone()]);

            if self.max.0 == key {
                if let Some((k, v)) = self.count_words.last_key_value() {
                    self.max = (v.first().unwrap().clone(), *k);
                } else {
                    self.max.1 -= 1;
                }
            }
        }
    }

    fn get_max_key(&self) -> String {
        return self.max.0.clone();
    }

    fn get_min_key(&self) -> String {
        return self.min.0.clone();
    }
}

fn main() {
    let mut all_one = AllOne::new();
    println!("{all_one:?}");
    all_one.inc(String::from("hello"));
    all_one.inc(String::from("world"));
    all_one.inc(String::from("leet"));
    all_one.inc(String::from("code"));
    all_one.inc(String::from("ds"));
    all_one.inc(String::from("leet"));
    println!("{all_one:?}");
    println!("{}", all_one.get_max_key());

    all_one.inc(String::from("ds"));
    all_one.dec(String::from("leet"));
    println!("{all_one:?}");
    println!("{}", all_one.get_max_key());
}
