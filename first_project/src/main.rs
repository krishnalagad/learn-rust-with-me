fn main() {
    let x = 10;
    println!("Value of x: {}", x);
    {
        let x = 11 + x;
        println!("Value of x: {}", x);
    }
    let x = 12;
    println!("Value of x: {}", x);

    const MINUTES: u32 = 59;
    println!("Value of const var: {}", MINUTES);

    
}
