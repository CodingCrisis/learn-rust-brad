pub fn run() {
    // Primitive value (array)
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    // Vector (not a primitive value)
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));
}
