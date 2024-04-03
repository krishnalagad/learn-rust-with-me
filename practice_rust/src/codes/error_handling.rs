use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
/*
    rustc error_handling.rs -o app && time ./app && rm app
 */
fn main() {
    let file = File::open("text.txt");
    // One approach of dealing with file
    let _file = match file {
        Ok(data) => data,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("text.txt") {
                Ok(file) => file,
                Err(error) => panic!("Error in creating file: {:#?}", error)
            },
            other_error => panic!("Error opening file: {:#?}", other_error)
        }
    };

    // Another approach of dealing with file
    let _file = File::open("file1.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("file1.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {:#?}", error)
            })
        } else {
            panic!("Problem opening the file: {:#?}", error)
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("file1.txt")?.read_to_string(&mut username)?;
    Ok(username)
}