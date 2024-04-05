/*
    rustc lifetime_rust.rs -o app && time ./app && rm app
 */
fn main() {
    let str1 = String::from("Krishna");
    let str2 = String::from("Lagad");

    let result = longest(str1.as_str(), str2.as_str());
    println!("Longest string: {}", result);
}
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}