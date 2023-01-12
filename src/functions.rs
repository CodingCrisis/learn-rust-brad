pub fn run() {
    greetings("Hello", "Marcin");
}

fn greetings(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name); 

    let get_sum = add(5, 7);
    println!("Sum: {}", get_sum);

    // closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(3, 7));
}

fn add(n1: i32, n2: i32) -> i32 {
    // no semicolon means return
    n1 + n2
}

