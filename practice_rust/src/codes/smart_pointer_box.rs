use std::fmt::Display;
use std::ops::Deref;
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

use List::{Cons, Nil};

struct MySmartPtr<T: Display> {
    data: T
}

impl <T: Display> Deref for MySmartPtr<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Display> Drop for MySmartPtr<T> {
    fn drop(&mut self) {
        println!("Dropping MySmartPtr with data: {}", self.data);
    }
}

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

    let my_ptr = MySmartPtr { data: 10 };
    println!("Value: {}", *my_ptr);

    let str1 = MySmartPtr { data: String::from("Krishna") };
    let str2 = MySmartPtr { data: String::from("Lagad") };
    println!("Smart pointers created!!");
}