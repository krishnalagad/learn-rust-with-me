use std::collections::HashMap;
/*
    rustc hashmap.rs -o app && time ./app && rm app
 */
fn main() {
    let blue = String::from("Blue");
    let red = String::from("Red");

    let mut scores_map = HashMap::new();
    scores_map.insert(blue, 99);    // ownership of key is transferred to hashmap now
    scores_map.insert(red, 100);    // ownership of key is transferred to hashmap now
    for (k, v) in &scores_map {
        println!("{} -> {}", k, v);
    }

    let search_key = String::from("Blue");
    if let Some(data) = scores_map.get(&search_key) {
        println!("Data is {}", data);
    }

    scores_map.insert("Pink".to_string(), 87);

    get_words_count();
}

fn get_words_count() {
    let mut map: HashMap<String, i32> = HashMap::new();
    let text_string = "Krishna lagad is my name. my job name is developer.";

    for word in text_string.split_whitespace() {
        // or_insert() return mutable reference of its parameter
        let count = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}