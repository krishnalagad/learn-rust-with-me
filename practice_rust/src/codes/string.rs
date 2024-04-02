/*
    rustc string.rs -o app && time ./app && rm app
 */
fn main() {
    let mut str1 = String::new();
    str1.push_str("Krishna");
    let str2 = String::from("Lagad");
    let str3 = str1 + &str2;
    println!("{}", str3);

    // println!("{}", str1);    // ownership moved to str3
    println!("{}", str2);   // temporarily borrowed while performing concat, after that its accessible

    let s1 = "Krishna";
    let s2 = "Lagad";
    let s3 = s1.to_owned() + s2;    // convert reference to owned String
    println!("{}", s3);

    let s = String::from("Krishna");
    let p = String::from("Lagad");
    let q = format!("{}{}", s, p);
    println!("{}", q);

}