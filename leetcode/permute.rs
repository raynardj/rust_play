// problem
// https://leetcode.com/problems/permutations/

// fastest solution

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 {
            return vec![];
        } else if nums.len() == 1 {
            return vec![nums];
        } else {
            let mut res = vec![];
            for i in 0..nums.len() {
                let mut tmp = nums.clone();
                tmp.remove(i);
                let mut tmp_res = Solution::permute(tmp);
                for j in 0..tmp_res.len() {
                    let mut tmp_res_j = tmp_res[j].clone();
                    tmp_res_j.insert(0, nums[i]);
                    res.push(tmp_res_j);
                }
            }
            res
        }
    }
}
