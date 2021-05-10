
fn use_slice(slice: &mut[i32]){
    println!("First element = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}



pub fn run(){
    let mut data = [1,2,3,4,5];
    use_slice(&mut data[1..4]); // not including 4
    println!("{:?}", data)
}