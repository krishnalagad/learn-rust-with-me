/*
    rustc functions.rs -o app && time ./app && rm app
 */
fn main() {
    println!("Project starts!!");
    test_function(12, 23);
    let result = add_numbers(23, 34);
    println!("Sum: {}", result);
    println!("Subtraction: {}", sub_numbers(9, 6));
}

fn test_function(a: i32, b: i32) {
    println!("Sum of numbers: {}", (a + b));
}

fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

fn sub_numbers(x: i8, y: i8) -> i8 {
    return x - y;
}