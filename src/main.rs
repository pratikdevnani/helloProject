mod compound;
mod primitive;
mod functions;

fn main() {
    println!("Hello, Rust from Cargo!");
    primitive::run();
    compound::run();
    functions::run();
}
