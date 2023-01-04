// Let's play with variables

pub fn run() {
    // Immutable
    let name = "Marcin";
    // Mutable
    let mut age = 41;

    println!("My name is {} and I am {}", name, age);
    age = 42;
    println!("Nah, I'm actually {}", age);

    // Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Multpile vars at once
    let (my_name, my_age) = ("Marcin", 42);
    println!("{} is {}", my_name, my_age);
}
