struct User {
    username: String,
    email: String,
    sign_in_count: u16,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    length: i32,
    width: i32
}

impl Rectangle {
    fn get_area_of_rect(&self) -> i32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

impl Rectangle {
    fn associate_function() {
        println!("Associate function of Rectangle struct!!");
    }
}
/*
    rustc structs.rs -o app && time ./app && rm app
 */
fn main() {
    let user1: User = User {
        username: String::from("krishnalagad"),
        email: String::from("krishna@mail.com"),
        sign_in_count: 1,
        active: true
    };
    println!("user1: {}", user1.username);

    let user2 = build_user(String::from("username"), String::from("email@mail.com"));
    println!("user2: {}", user2.active);
    println!("user2: {}", user2.sign_in_count);

    // We can create User struct instance using partial data of already existing User struct instance
    let user3 = User {
        username: String::from("user3_username"),
        ..user1
    };
    println!("user3: {}", user3.email);

    let rect1 = Rectangle {
        length: 12,
        width: 7
    };
    let rect2 = Rectangle {
        length: 30,
        width: 20
    };
    println!("{:#?}\n{:#?}", rect1, rect2);
    println!("Area of rectangle: {}", rect1.get_area_of_rect());
    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1: {}", rect2.can_hold(&rect1));
    Rectangle::associate_function()
}

fn build_user(username: String, email: String) -> User {
    User {
        username,       // this is called as struct init shorthand syntax
        email,          // only possible if param names and struct field names are exactly same
        sign_in_count: 1,
        active: true
    }
}

