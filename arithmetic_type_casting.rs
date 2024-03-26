/*
    rustc arithmetic_type_casting.rs -o app && time ./app && rm app
 */
fn main() {
    let x: u8 = 12;
    let y: u8 = 10;

    let z: f32 = (x / y).into();
    let _ans: u16 = (x * y).into();
    println!("Result: {}", z);

    let num1 = 12 as u8;
    let num2 = 8 as u8;
    println!("Result: {}", (num1 + num2));
}