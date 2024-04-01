pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_wishlist() {}
        pub fn seat_at_table() {}
    }
    pub mod serving {
        pub fn take_order() { println!("Take order!!"); }
        pub fn serve_order() {}
        pub fn take_payment() {}
    }
}

fn go_to_restaurant() {
    front_of_house::serving::take_payment();
}

pub mod food {
    fn eat_food() {
        super::go_to_restaurant();  // we can access function in parent module using super keyword.
    }
}