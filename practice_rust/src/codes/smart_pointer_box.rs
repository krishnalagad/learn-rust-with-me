#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

use List::{Cons, Nil};
/*
    rustc smart_pointer_box.rs -o app && time ./app && rm app
 */
fn main() {
    let list = List::Cons(10, Box::new(List::Cons(20, Box::new(List::Cons(30, Box::new(Nil))))));
    println!("{:?}", list);

    let x = 10;
    let y = Box::new(x);

    assert_eq!(x, 10);
    assert_eq!(x, *y);
}