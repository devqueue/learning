// Enums are types which have a few definite values

enum Movement{
    // Varients
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement){
    // Perform action depending on info
    match m{
        Movement::Up => println!("Avatar Moving up"),
        Movement::Down => println!("Avatar Moving Down"),
        Movement::Left => println!("Avatar Moving Left"),
        Movement::Right => println!("Avatar Moving Right"),
 
    }
}


pub fn run(){
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Right;
    let avatar3 = Movement::Down;
    let avatar4 = Movement::Up;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}