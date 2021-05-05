// Structs - Used to create custom data types

// traditional struct
struct Color{
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct ColorTuple(u8, u8, u8);

// Person struct
struct Person{
    first_name: String,
    last_name: String
} 

impl Person{
    // costruct person
    fn new(first: &str, last: &str) -> Person {
        Person{
            first_name: first.to_string(),
             last_name: last.to_string()
            }
    }

    // Get full name
    fn full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn set_ln(&mut self, last: &str){
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn name_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }

}



pub fn run(){
    let mut color = Color{red:255, green:0, blue:0};
    color.green = 255; 
    println!("COLOR: {} {} {}", color.red, color.green, color.blue);

    let mut color_tuple = ColorTuple(0, 255, 255);
    color_tuple.0 = 200;
    println!("Color {} {} {}", color_tuple.0, color_tuple.1, color_tuple.2);

    let mut p = Person::new("john", "Doe");
    println!("Person: {} {}", p.first_name, p.last_name);
    p.set_ln("will");
    println!("Person: {}", p.full_name());
    println!("Person: {:?}", p.name_tuple());

}