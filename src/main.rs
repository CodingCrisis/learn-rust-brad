mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;

fn main() {
    println!("Hello, world!");
    println!();
    print::run();
    println!();
    vars::run();
    println!();
    types::run();
    println!();
    strings::run();
    println!();
    tuples::run();
    println!();
    arrays::run();
    println!();
    vectors::run();
    println!();
    conditionals::run();
    println!();
    loops::run();
}
