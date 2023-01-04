mod print;
mod vars;
mod types;

fn main() {
    println!("Hello, world!");
    println!();
    print::run();
    println!();
    vars::run();
    println!();
    types::run();
}
