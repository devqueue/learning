// Conditionals - Used to check the condition of something and act on the result

pub fn run() {
    let age = 18;
    let check_id: bool = false;
    let knows_person = true;

    // if - else

    if age >=21 && check_id || knows_person{
        println!("Bartender: What would you like to drink?");
    }
    else if age < 21 && check_id {
        println!("Bartender: Sorry you have to leave");
    }
    else{
        println!("Bartender: show your ID");
    }

    // short hand if
    let is_of_age = if age>=21 {true} else {false};
    println!("IS OF AGE: {}", is_of_age);

}