/*
    rustc data_types.rs && time ./data_types && rm data_types
    rustc data_types.rs -o app && time ./app && rm app
*/
fn main() {
    // integer data types in rust
    let x: u8 = 123;
    println!("unsigned int: {}", x);

    let x: i8 = -12;
    println!("signed int: {}", x);
}