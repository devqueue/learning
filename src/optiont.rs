pub fn run(){
    // option<T>
    let x = 3.0;
    let y = 0.0;
    let result:Option<f64> =
        if y != 0.0 {Some(x/y) } else {None};

    // at any time option is one of 2 possible states, Some(z) or None, Some of z means there is some value availible and none means no value is availible

    println!("{:?}", result);

    // instead we can also 
    match result{
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("Cannot divide {} by {}", x, y)
    }

    // if let or while let
    if let Some(z) = result{
        println!("Z = {}", z);
    }
    // if will only be executed if the destructuring was sucessfull i.e the result is not None (same can be aplied with a while loop as well) 
}