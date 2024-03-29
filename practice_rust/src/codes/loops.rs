/*
    rustc loops.rs -o app && time ./app && rm app
 */

fn main() {
    println!("Welcome!!");

    let mut counter = 0;
    let mut result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("Loop counter: {}", result);

    print!("LIFO: ");
    while result != 0 {
        print!("{}  ", result);
        result -= 1;
    }
    println!();

    // For loops
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    print!("For loop on collection: ");
    for element in arr.iter() {
        print!("{}  ", element);
    }
    println!();

    print!("For loop on range: ");
    for number in 1..=p30 {
        print!("{} ", number);
    }
    println!();
}