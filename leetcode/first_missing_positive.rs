// problem
// https://leetcode.com/problems/first-missing-positive/

// using a HashMap to store the numbers as keys
// use std::collections::HashMap;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut hashmap = HashMap::new();
        for i in 0..nums.len() {
            if nums[i] >= 1 {
                hashmap.insert(nums[i] as usize, true);
            }
        }
        for i in 1..=hashmap.len()+1 {
            if !hashmap.contains_key(&i) {
                return i as i32;
            }
        }
        nums.len() as i32 + 1
    }
}
