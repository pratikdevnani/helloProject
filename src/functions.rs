// public facing function
pub fn run(){
    self::main();
}

// module function
fn hello_world(){
    println!("Hello from inside rust function");
}

// function that takes a val
fn tell_height(height: u32){
    println!("My height is {} cm", height);
}

//  functions that take more vals
fn human_id(name: &str, age: u32, height: f32){
    println!("{}'s age is {} and height is {}", name, age, height);
}

// function returning values
fn add(a: i32, b: i32) -> i32{
    a + b
}

// Entrypoint
fn main(){
    println!("Hello, World");
    self::hello_world();
    self::tell_height(182);
    self::human_id(&"Pratik", 27, 157.3);
    let cost = {
        let price: u32 = 24;
        let quantity: u32 = 10;
        price * quantity
    };
    println!("The total cost is {}", cost);
    let sum = self::add(5,5);
    println!("The summation is {}", sum);
}

// Expression - returns a value - 5, true & false, add(a, b), if condition {value1} else {value2}
// Statement - does not return a value - variable declarations, function definitions, control flow statements