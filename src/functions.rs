pub fn run(){
    greeting("Hello", "Alice");

    let get_sum = add(5, 5);
    println!("5 + 5 = {}", get_sum);

    // inline function
    let add_nums = |n1:i32, n2:i32| n1+n2;

    println!("3 + 3 = {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{greet} {name}, nice to meet u", greet=greet, name=name);
}

fn add(n1: i32, n2: i32) -> i32 {
    // if we don't use ; at the end of the statement, the return value will be the last statement
    n1 + n2
}