// #[derive(Debug)]
// enum Option<T> {    // this is how built-in Option enums looks like.
//     Some(T),
//     None,
// }

#[derive(Debug)]
enum State {
    Maharastra,
    UP,
    Gujrat
}

enum Coin {
    Paisa,
    Rupaya,
    Ana(State)
}
/*
    rustc option_enum.rs -o app && time ./app && rm app
 */
fn main() {
    // let some_integer = Option::Some(110);
    // let some_string = Option::Some("krishna");
    // let absent_value: Option<i32> = Option::None;
    // println!("{:?}\n{:?}\n{:?}", some_integer, some_string, absent_value);

    let a = Some(32);
    let str = Some("Lagad");
    let ab: Option<String> = None;
    println!("{:?}\n{:?}\n{:?}", a, str, ab);

    let num1: Option<i32> = Some(10);
    let num2 = 20;

    let result = num1.unwrap_or(0) + num2;
    println!("Result: {}", result);

    let result = coin_to_value(Coin::Ana(State::Gujrat));
    println!("Coin value: {}", result);

    let five_v = Some(5);
    let six_v = plus_one(Some(5));
    let seven_v = plus_one(Some(6));
    let absent_v = plus_one(None);

    println!("Five: {:?}, Six: {:?}, Seven: {:?}, Absent: {:?}", five_v, six_v, seven_v, absent_v);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None,      // for any type of value other than Option<u32>
    }
}

fn coin_to_value (coin: Coin) -> u32 {
    match coin {
        Coin::Paisa => 1,
        Coin::Rupaya => 100,
        Coin::Ana(state) => {
            match state {
                State::Maharastra => {
                    println!("Ana State: {:?}", state);
                    7
                },State::UP => {
                    println!("Ana State: {:?}", state);
                    5
                },State::Gujrat => {
                    println!("Ana State: {:?}", state);
                    6
                },
            }
        }
    }
}

