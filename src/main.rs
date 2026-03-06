mod compound;
mod functions;
mod ownership;
mod primitive;

fn main() {
    println!("Hello, Rust from Cargo!");
    primitive::run();
    compound::run();
    functions::run();
    ownership::run();
}
