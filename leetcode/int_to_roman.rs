pub fn to_roman(num: i32, digit: usize) -> String {
    let letter_map = [
        ['I', 'V', 'X'],
        ['X', 'L', 'C'],
        ['C', 'D', 'M'],
        ['M', 'M', 'M'],
    ];
    let chars = letter_map[digit as usize];

    let mut result = String::new();

    if num <= 3 {
        result = chars[0].to_string().repeat(num as usize);
        result
    } else if num == 4 {
        result = chars[0].to_string() + &chars[1].to_string();
        result
    } else if num == 5 {
        result = chars[1].to_string();
        result
    } else if num <= 8 {
        result = chars[1].to_string() + &chars[0].to_string().repeat(num as usize - 5);
        result
    } else if num == 9 {
        result = chars[0].to_string() + &chars[2].to_string();
        result
    } else {
        panic!("Invalid number");
    }
}

pub fn digit_array(num: i32) -> Vec<i32> {
    // return a vector of digits
    let mut digits: Vec<i32> = Vec::new();
    let mut num = num;
    while num > 0 {
        digits.push(num % 10);
        num = num / 10;
    }
    digits
}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut result = String::new();
        let mut num = num;
        let mut digits = digit_array(num);

        for (i, &n) in digits.iter().enumerate() {
            result = to_roman(n, i) + &result;
        }
        result
    }
}
