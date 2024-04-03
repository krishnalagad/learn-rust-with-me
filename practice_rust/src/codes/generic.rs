struct Point<T, U> {
    x: T,
    y: U
}
/*
    rustc generic.rs -o app && time ./app && rm app
 */
fn main() {
    let v1 = vec![10,23, 1, 7, 98];
    let result = get_largest(&v1);
    println!("Largest element : {}", result);

    let v2 = vec!['k', 'r', 'i', 's', 'h', 'n', 'a'];
    let result = get_largest(&v2);
    println!("Largest character: {}", result);

    let pt1 = Point {x: 10, y: 11};
    let pt2 = Point {x: 10.12, y: 11.23};
    let pt3 = Point {x: 10, y: 11.98};
}

fn get_largest<T: PartialOrd + Copy>(v: &Vec<T>) -> T {
    let mut largest = v[0];
    for num in v {
        if *num > largest {
            largest = *num;
        }
    }
    largest
}