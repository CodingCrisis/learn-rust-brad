// Working with Strings
// Two types - 1) primitive, immutable; 2) growable, heap-allocated

pub fn run(){
    let hello_prim = "Hello"; 
    let mut hello_grow = String::from("Hello");
    let mut hello_cap = String::with_capacity(10);

    // Push char
    hello_grow.push('2');
    hello_cap.push('a');

    println!("Primitive str: {}, Growable str: {}", hello_prim, hello_grow);

    // Push string
    hello_grow.push_str(" World!");
    println!("{}", hello_grow);

    println!("String length: {}", hello_grow.len());
    println!("String capacity: {}", hello_grow.capacity());
    println!("Is empty: {}", hello_grow.is_empty());
    println!("Contains word 'World': {}", hello_grow.contains("World"));
    println!("Replace: {}", hello_grow.replace("2", ""));

    for word in hello_grow.split_whitespace(){
        println!("{}", word);
    }

    println!("{}", hello_cap);

    // Assertion testing <-- that was quite random Brad :)
    assert_eq!(1, hello_cap.len());
    assert_eq!(10, hello_cap.capacity());
}
