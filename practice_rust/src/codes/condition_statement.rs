/*
    rustc condition_statement.rs -o app && time ./app && rm app
 */
fn main() {
    let food = "bread";
    if food == "fruit" {
        println!("{}", food);
    } else if food == "bread" {
        println!("{}", food);
    } else {
        println!("{}", food);
    }

    let condition: bool = false;
    let number = if condition { 10 } else { 20 };
    println!("Number is: {}", number);
}
