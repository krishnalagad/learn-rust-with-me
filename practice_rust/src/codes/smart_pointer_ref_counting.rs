use std::rc::Rc;
#[derive(Debug)]
enum List {
    Con(i32, Rc<List>),
    Nil
}

use List::{Con, Nil};

/*
    rustc smart_pointer_ref_counting.rs -o app && time ./app && rm app
 */
fn main() {
    let a = Rc::new(List::Con(10, Rc::new(List::Con(20, Rc::new(List::Nil)))));
    println!("Reference count of a: {}", Rc::strong_count(&a));

    let b = List::Con(2, Rc::clone(&a));
    println!("Reference count of a: {}", Rc::strong_count(&a));
    // inner scope
    {
        let c = List::Con(5, Rc::clone(&a));
        println!("Reference count of a in inner scope: {}", Rc::strong_count(&a));
    }
    println!("Final reference count of a: {}", Rc::strong_count(&a));
}