use std::io;

/*
    rustc console_input.rs -o app && time ./app && rm app
*/
fn main() {
    println!("Enter input: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line!!");
    println!("{}", input);
}