// Ownership, borrowing and references
// Ownership:
// C, C++ -> memoery management control issue
// Garbage Collection -> causes hangs or pauses in execution if not truly concurrent (slow performance)

// Ownership Rules
// 1. Each value in Rust has a variable that is its owner
// 2. There can only be 1 owner at a time
// 3. When the owner goes out of scope, that value will be dropped

pub fn run() {
    self::main();
}

fn main() {
    let s1 = String::from("RUST");
    let len = self::calculate_len(&s1);
    println!("Lenght of {} is {}", s1, len);
    // Transferring ownership of string s1
    let s2 = s1;
    println!("String {}", s2);
}

// out here, the reference s1 does not exist as it is out of scope

fn calculate_len(s: &String) -> usize {
    s.len()
}
