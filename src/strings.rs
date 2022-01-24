pub fn run(){
    let mut hello = String::from("Hello");
    hello.push('W');
    hello.push_str(", space, hahahahhahahhah");

    // capacity in bytes
    println!("{}, length:{}", hello, hello.len());
    println!("{}, capacity:{}", hello, hello.capacity());
    // loop through string by white space

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(20);
    s.push('a');
    s.push('b');
    println!("{}, length:{}", s, s.len());
}