/*
    rustc references.rs -o app && time ./app && rm app
 */
fn main() {
    let mut str: String = String::from("Krishna");
    let len = get_length(&str);
    println!("Length of string {} is {}", str, len);

    change_string(&mut str);
    println!("changed string: {}", str);

    let mut a: String = String::from("Lagad");
    // We can have multiple immutable references to a piece of data,
    // but only one mutable reference at a time.
    let ref1 = &a;
    let ref2 = &a;
    let ref3 = &mut a;
}

fn get_length(str: &String) -> usize {
    let len = str.len();
    len
}

fn change_string(str: &mut String) {
    str.push_str("Lagad");
}