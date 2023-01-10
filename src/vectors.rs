use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("Vector: {:?}", numbers);

    println!("Single value: {}", numbers[0]);

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(6);
    numbers.push(7);

    // Pop off last number
    numbers.pop();

    println!("{:?}", numbers);

    // Vector length
    println!("Length: {}", numbers.len());

    // Arrays live on stack
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Vector after mutating loop: {:?}", numbers);
}
