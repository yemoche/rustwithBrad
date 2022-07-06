
pub fn run() {
    let age: u8 = 30;
    let check_id: bool = true;

    //if/Else
    if age >= 21 && check_id {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave!");
    } else {
        println!("Bartender: I will need to check your ID");
    }

    //shorthand if
    let is_of_age = if age >= 21 {true} else { false }; //If age is greater than or equal to 212, set to true
    println!("Is of Age: {}", is_of_age);
}