// solution for this problem:
// https://leetcode.com/problems/missing-number/

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut nums2 = nums.clone();
        nums2.sort();

        return Self::recur_find_missing(&nums2, 0, nums2.len() - 1);
    }
    pub fn recur_find_missing(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        // A btree recurssion to find the missing number
        if start == end {
            if nums[start] != start as i32 {
                return start as i32;
            } else{
                return start as i32 + 1;
            }
        }
        let mid = (start + end) / 2;
        if nums[mid] == mid as i32 {
            return Self::recur_find_missing(nums, mid + 1, end);
        } else {
            return Self::recur_find_missing(nums, start, mid);
        }
    }
}