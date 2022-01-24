pub fn run(){
    let age = 12;
    let is_of_age = if age > 18 {
        true
    } else {
        false
    };
    println!("Is of age: {}", is_of_age);
}