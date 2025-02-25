use std::collections::HashSet;
pub fn new_count_distinct(input_str: &str) -> usize {
    let s1: HashSet<&str> = input_str.split(',').into_iter().collect();
    s1.len()
}
