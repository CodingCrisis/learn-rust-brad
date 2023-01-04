// Working with primitive types

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Explicit type
    let z: i64 = 9999999999998;

    // Max size of types
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;
    let is_active_explicite: bool = true;
    let is_greater = 10 < 5;

    println!("{:?}", (x, y, z, is_active, is_active_explicite, is_greater));

    // Characters and emoji :)
    let a1 = 'M';
    let face = '\u{1F600}';

    println!("{:?}", (a1, face));
}    

