pub fn run(country_code: i32){
    let country = match country_code{
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=999 => "unknown", //inclusive range 1..99 will not include 99
        _ => "Invalid"
    };
    println!("Country: {}", country)
}