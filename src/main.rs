mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;

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
}
