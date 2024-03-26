use std::io;

/*
    rustc parse_numbers.rs -o app && time ./app && rm app
*/
fn main() {
    println!("Enter numbers: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Expect to get input");
    let int_input: i64 = input.trim().parse().unwrap();
    println!("Parsed integer: {}", int_input);
}