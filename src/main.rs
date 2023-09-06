use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {

    let fizzish = "fizz";
    let result = foo_if_fizz(fizzish);
    println!("{}", result);
    // Path to the desired file

    let path = Path::new("hell.txt");
    let display = path.display();

    // Open the path in read-only mode, returns 'io::Result <File>'
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns 'io::Result<usize>'
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains: \n{}", display, s),
    }

    // 'file' goes out of scope, and the hell.txt file gets closed
}

pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz"{
        "foo"
    } else if fizzish == "fuzz"{
        "bar"
    } else {
        "baz"
    }
}
