use core::fmt;
use std::fs::File;
use std::i32;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
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

    let mut total_zeros = 0;
    let mut lock_position = 50; // can be between 0 and 99
    for (i, part) in parts.enumerate() {
        if part.is_empty() {
            continue;
        }

        println!("processing part #{}: {}", i, part);
        let direction = extract_direction(part);
        let distance = extract_distance(part);
        println!("Direction: {}, with distance: {}", direction, distance);
        lock_position += process_lock_part(lock_position, direction, distance);
        if lock_position == 0 {
            total_zeros += 1;
            println!("found a zero at position: {}", lock_position);
        }
    }

    println!("total zeros found: {}", total_zeros);
}

fn process_lock_part(mut lock_position: i32, direction: Direction, distance: i16) -> i32 {
    lock_position += get_movement_amount(direction, distance); // returned negative for left, positive for right
    lock_position %= 100; // is this right for the wrap around? 
    println!("lock position: {}", lock_position);
    lock_position
}

fn get_movement_amount(direction: Direction, distance: i16) -> i32 {
    if Direction::Left.to_string() == direction.to_string() {
        -(distance as i32)
    } else {
        distance as i32
    }
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
