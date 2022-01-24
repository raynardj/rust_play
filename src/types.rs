pub fn run(){
    let x=10;
    let y=16.8;
    let z: i64 = 709879870;
    let is_active: bool = true;
    println!("max i32: {}", std::i32::MAX);
    println!("max i64: {}", std::i64::MAX);

    println!("({:?})", (x, y, z, is_active));
}