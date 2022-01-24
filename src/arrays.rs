pub fn run(){
    let numbers:[i16; 10] = [1,2,3,4,5,6,7,8,9,10];
    println!("{:?}", numbers);

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    for i in 0..numbers.len(){
        println!("{idx}:{number}", number=numbers[i], idx=i);
    }

    let slice: &[i16] = &numbers[0..5];
    println!("Slice: {:?}", slice);
}