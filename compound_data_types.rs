/*
    rustc compound_data_types.rs -o app && time ./app && rm app
*/
fn main() {
    let tup: (i8, bool, char) = (9, true, 'K');
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);

    let tup = ("krishna", 78, 'K', true);
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
    println!("{}", tup.3);
}