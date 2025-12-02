use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("./input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("file extracted successfully\n"),
    }

    let parts = s.split("\n");

    // println!("processing parts into data structure");
    // TODO: make a list of objects from parts. { direction: Direction, distance: i32 }
    for part in parts {
        println!("processing part: {}", part)
    }

    // println!("begin secret password calculation");
    let total_zeros = 0;
}

// TODO: add taking out first character to this
fn extract_direction(dir: &str) -> Direction {
    match dir {
        "L" => Direction::Left,
        "R" => Direction::Right,
        &_ => todo!(),
    }
}

enum Direction {
    Left,
    Right,
}
