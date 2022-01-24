pub fn run(){
    let mut counter = 0;
    // Infinite loop
    // loop {
    //     counter += 1;
    //     println!("Counter: {}", counter);
    //     if counter == 10 {
    //         break;
    //     }
    // }

    // While loop
    // while counter <=100 {
    //     if counter % 15==0 {
    //         println!("FizzBuzz");
    //     } else if counter % 3 ==0{
    //         println!("Fizz");
    //     } else if counter % 5 ==0{
    //         println!("Buzz");
    //     } else {
    //         println!("{}", counter);
    //     }
    //     counter+=1;
    // }

    // for range loop
    for x in 0..100 {
        if x % 15==0 {
            println!("FizzBuzz");
        } else if x % 3 ==0{
            println!("Fizz");
        } else if x % 5 ==0{
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }

}