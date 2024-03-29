/*
    rustc match_construct.rs -o app && time ./app && rm app
*/
fn main() {
    let number = 50;
    match number {
        // Match a specific value
        0 => println!("Number is zero!!"),
        // Match a range of values
        1..=50 => println!("Number is between 1 and 50!!"),
        // Match multiple specific values
        51 | 52 | 53 => println!("Number is either 51, 52, 53!!"),
        // Match a value and bind it to a variable
        num @ 54..=100 => println!("The number is {} and it's between 54 and 100", num),
        // Match any other value
        _ => println!("Number is greater than 100!!")
    }
}