/*
    rustc scaler_data_types.rs && time ./scaler_data_types && rm scaler_data_types
    rustc scaler_data_types.rs -o app && time ./app && rm app
*/
fn main() {
    // integer data types in rust
    let x: u8 = 123;
    println!("unsigned int: {}", x);

    let x: i8 = -12;
    println!("signed int: {}", x);

    // floating points in rust
    let floating_point: f32 = 3214.56;
    println!("float number: {}", floating_point);

    let letter: char = 'K';
    println!("Character: {}", letter);

    let name: &str = "Krishna";
    println!("{}", name);

}