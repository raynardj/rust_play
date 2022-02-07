//problem
//https://leetcode.com/problems/rotate-image/
// fastest solution by 2022-02-07

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let total: usize = matrix.len();
        let mut halfx: usize = total / 2;
        let mut halfy: usize = total / 2;
        if halfx*2 <total{
            // if total is odd number
            // halfx will include the middle number
            // but halfy won't
            halfx += 1;
        }
        let mut next_x: usize;
        let mut next_y: usize;
        let mut this_x: usize;
        let mut this_y: usize;
        let mut from_val: i32;
        let mut to_val: i32;

        for x in 0..halfx {
            for y in 0..halfy {
                this_x = x;
                this_y = y;
                to_val = matrix[this_y][this_x];
                for i in 0..4 {
                    // calculate the next position
                    next_x = total - 1 - this_y;
                    next_y = this_x;
                    // save a value from the next position
                    from_val = matrix[next_y][next_x];
                    // assign the value to overwrite the next position
                    matrix[next_y][next_x] = to_val;
                    // move to the next position
                    this_x = next_x;
                    this_y = next_y;
                    // pass on the value
                    to_val = from_val;
                }
            }
        }
    }
}
