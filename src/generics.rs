struct Point<T,V>{
    x:T,
    y:V
}

// both start end must be same type of data
struct Line<T>{
    start: Point<T, T>,
    end: Point<T, T>
}

// explicit definitions are not necessary
fn generics(){
    let _a:Point<i32, i32> = Point{x:0, y:0};
    let _b:Point<f64, f64> = Point{x:1.2, y: 3.4};
}

pub fn run(){

}