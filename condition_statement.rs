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
}
