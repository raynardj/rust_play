// promblem
// https://leetcode.com/problems/count-and-say/

// this is th fastest solution so far

impl Solution {
    pub fn say(s: String) -> String {
        let chars = s.chars().collect::<Vec<char>>();
        let mut count = 1;
        let mut result = String::new();
        for i in (0..s.len()-1){
            if chars[i] == chars[i+1]{
                count += 1;
            }else{
                result.push_str(format!("{}{}", count, chars[i]).as_str());
                count = 1;
            }
        }
        result.push_str(format!("{}{}", count, chars[s.len()-1]).as_str());
        result
    }
    pub fn count_and_say(n: i32) -> String {
        let mut result = String::from("1");
        for i in 1..n{
            result = Solution::say(result);
        }
        result
    }
}