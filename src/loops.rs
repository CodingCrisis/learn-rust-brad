pub fn run() {
    let mut count = 0;

    // Infinite loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    }

    count = 0;

    // While loop (fizz buzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }

        count += 1;
   }

    count = 0;

    // For loop
    for count in 0..100 {
         if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }
    }
}
