// problem
// https://leetcode.com/problems/h-index/

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut cit = citations.clone();
        cit.sort();
        let mut idx:usize;
        let mut h:i32 = 0;
        for (i, &c) in cit.iter().rev().enumerate() {
            h = (cit.len() - i) as i32;
            idx = cit.len() - (h as usize);
            if idx >= cit.len(){
                continue;
            }
            if cit[idx] >= h {
                return h;
            }
        }
        return 0;
    }
}