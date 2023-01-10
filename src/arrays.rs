use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Array: {:?}", numbers);

    println!("Single value: {}", numbers[0]);

    // Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Array length
    println!("Length: {}", numbers.len());

    // Arrays live on stack
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}
