pub fn scope_shadowing(){
    let a = 123;
    // Any set of curly braces can start a scope
    {
        let b = 435;
        println!("b = {}", b);
        println!("outside a = {}", a); // outside variable availible inside
        let a = 324;
        println!("inside a = {}", a); // inside var shadowing the outside
    }
    // println!("outside b = {}", b);
    println!("outside a = {}", a);

}
