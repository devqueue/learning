// Enums are types which have a few definite values

enum Movement{
    // Varients
    Up,
    Down,
    Left,
    Right,
    Uplrmov(u8, u8, u8, u8),
    Cmyk{cyan:u8, magenta: u8,  yellow:u8, black: u8}
}

fn move_avatar(m: Movement){
    // Perform action depending on info
    match m{
        Movement::Up => println!("Avatar Moving up"),
        Movement::Down => println!("Avatar Moving Down"),
        Movement::Left => println!("Avatar Moving Left"),
        Movement::Right => println!("Avatar Moving Right"),
        Movement::Uplrmov(0,0,0,0) 
        | Movement::Cmyk{cyan:0, magenta: 0, yellow: 0, black: 0}=> println!("Dont move"), // movement 0 or colors 0 => print dont move
        Movement::Uplrmov(u, p, l, r) => println!("Move {} {} {} {} respectively", u, p, l, r),
        _ => () // anything else => do nothing
 
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