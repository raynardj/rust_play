pub fn run(){
    let person: (&str, &str, i8) = ("Ray", "Jon", 33);
    println!("{} {} is {}", person.0, person.1, person.2);
}