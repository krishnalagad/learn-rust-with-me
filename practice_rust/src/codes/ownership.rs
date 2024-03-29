/*
   rustc ownership.rs -o app && time ./app && rm app
*/
fn main() {
    let x = 123;
    let y = x; // copy
    println!("x: {}, y: {}", x, y);

    let str1: String = String::from("Krishna");
    let str2: String = str1;
    println!("str2: {}", str2); // str1 is not accessible, it has been moved.

    let str1: String = String::from("Krishna");
    let str2: String = str1.clone();
    println!("str1: {}, str2: {}", str1, str2); // str1 is accessible here, it has been cloned.

    println!("\nOwnership Transfer\n");

    let str3: String = String::from("Krishna");
    takes_ownership(str3); // now ownership of str3 has passed to scope of fn takes_ownership.

    let str4: String = gives_ownership(); // get the ownership of str in gives_ownership in main_scope.
    println!("In main_scope from gives_ownership_scope: {}", str4);

    let str5: String = takes_and_gives_ownership(str4);
    println!("In main_scope from takes_and_gives_ownership_scope: {}", str5);
}

fn takes_ownership(str: String) {
    println!("In takes_ownership_scope from main_scope: {}", str);
}
fn gives_ownership() -> String {
    let str: String = String::from("Krishna");
    str
}
fn takes_and_gives_ownership(str: String) -> String {
    println!("In takes_and_gives_ownership_scope from main_scope: {}", str);
    let modified_str = str.to_uppercase();
    modified_str
}
