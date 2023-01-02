pub fn run(){
    //Hello world
    println!("Hello, from a function!");
    
    //Print with args
    println!("{} is playing with {}", "Marcin", "Rust");
    
    //Print with positional args
    println!("{0} is playing with {1} arguments in {2}", "Marcin", "positional", "Rust");

    //Print with named args
    println!(
        "{name} is playing with {typeofarg} arguments in {language}",
        name = "Marcin",
        typeofarg = "named",
        language = "Rust"
    );

    //Print with number formatting
    println!("{0} in Hex is \"{0:X}\", but \"{0:o}\" in octal", 10);

    //Print a tuple
    println!("{:?}", ("Marcin", true, 41));
    
    //Print with basic math
    println!("10 + 20 = {}", 10 + 20);
}
