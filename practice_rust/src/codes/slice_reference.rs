/*
    rustc slice_reference.rs -o app && time ./app && rm app
 */
fn main() {
    let mut str = String::from("Krishna Lagad is my name.");
    let s = "Krishna Lagad";

    let word = first_word_length(&mut str);
    println!("Length of first word in String: {}", word);

    let word = first_word(s);
    println!("{}", s);      // accessible here because its reference and mutability is on.
    println!("First word in String: {}", word);
}

fn first_word_length(str: &mut String) -> usize {
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    str.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}