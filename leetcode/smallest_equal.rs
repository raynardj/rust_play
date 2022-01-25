impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        let mut mod_ :i32;
        let mut idx:i32=-1;
        for (i, &n) in nums.iter().enumerate(){
            idx = i as i32;
            mod_ = idx%10;
            if(mod_==n){
                return idx;
            }
        }
        -1
    }
}