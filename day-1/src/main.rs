use core::fmt;
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

    for (i, part) in parts.enumerate() {
        if part.is_empty() {
            continue;
        }

        println!("processing part #{}: {}", i, part);
        let direction = extract_direction(part);
        let distance = extract_distance(part);
        println!("Direction: {}, with distance: {}", direction, distance);
    }

    // println!("begin secret password calculation");
    // let total_zeros = 0;
}

enum Direction {
    Left,
    Right,
}

fn extract_direction(dir: &str) -> Direction {
    let ch = dir.chars().next().unwrap();
    match ch {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => todo!(),
    }
}

fn extract_distance(dir: &str) -> i16 {
    let slice = &dir[1..];
    slice.parse::<i16>().unwrap() // apparently we dont ened returns in rust? thats crazy
}

impl fmt::Display for Direction {
    // need to do this to let the rust print it
    // https://users.rust-lang.org/t/how-can-i-implement-fmt-display-for-enum/24111
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),
        }
    }
}
