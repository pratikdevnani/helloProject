pub fn run(){
    // Arrays
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Numbers in the array are {:?}", numbers);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("The fruits are {:?}", fruits);
    println!("The first fruit is {}", fruits[0]);
    println!("The second fruit is {}", fruits[1]);
    println!("The third fruit is {}", fruits[2]);

    // Tuple
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human information = {:?}", human);

    let animal= ("Kemper", 30, false, [1,2,3,4,5]);
    println!("Animal information = {:?}", animal);

    // Slices
    let number_slice: &[i32; 5] = &[1,2,3,4,5];
    println!("Values in slices = {:?}", number_slice);

    let animal_slice: &[&str; 3] = &["tiger", "cat", "dog"];
    println!("Values in animal slice = {:?}", animal_slice);

    let string_slice: &[&String; 3] = &[
        &"harry".to_string(),
        &"ron".to_string(),
        &"hermionie".to_string()];
    println!("Values in string slice = {:?}", string_slice);

    // Strings
    // Growable, Mutable, Owned String type - allocated on the heap -> dynamic

    let mut stone_cold: String = String::from("Hell, ");
    println!("Values in String is {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Values in String after pushing is {}", stone_cold);

    // B- &str (String slice)
    // reference to string somewhere in code - on stack - immutable

    let string: String = String::from("Hello World!");
    // taking a reference to the variable that is owned and only a sub-part of it
    let slice: &str = &string[0..5];
    println!("Slice value = {}", slice);
}