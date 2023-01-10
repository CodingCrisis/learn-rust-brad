pub fn run() {
    let age: u8 = 17;
    let check_id: bool = true;
    let knows_person_of_age = true;

    if age >= 18 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 18 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // No ternary operator, but one liners are fine like this:
    let is_of_age = if age >= 18 { true } else { false };

    println!("Is of age: {}", is_of_age);
}
