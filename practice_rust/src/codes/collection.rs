/*
    rustc collection.rs -o app && time ./app && rm app
 */
fn main() {
    // Approach 1 of creating vector
    let mut data: Vec<i32> = Vec::new();
    data.push(10);
    data.push(20);
    data.push(30);
    println!("{}", &data[0]);

    // Approach 2 of creating vector using vec!
    let v = vec![1, 2, 3, 4];
    let index = 20;
    match v.get(index) {
        Some(val) => println!("Value at index {} is {}.", index, val),
        None => println!("There is no element at index {}.", index)
    }

    enum Cell {
        Int(u32),
        Float(f32),
        Text(String)
    }

    let rows = vec![Cell::Int(10), Cell::Float(32.34), Cell::Text(String::from("Krishna"))];
    match &rows[2] {
        Cell::Text(str) => println!("Its String: {}", str),
        Cell::Int(val) => println!("Its Integer: {}", val),
        _ => println!("Its Float")
    }
}