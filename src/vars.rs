pub fn run() {
    let name = "Ray";
    let mut age = 30;
    age = 28;
    println!("My name is {name}, I'm {age}", name=name, age=age);

    let (my_name, my_age) = ("Ray", 30);
    println!("My name is {name}, I'm {age}", name=my_name, age=my_age);
}