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
        (lock_position, total_zeros) =
            process_lock_movement(lock_position, direction, distance, total_zeros);
    }

    println!("total zeros found: {}", total_zeros);
}

fn process_lock_movement(
    lock_position: i32,
    direction: Direction,
    distance: i16,
    mut total_zeros: i32,
) -> (i32, i32) {
    let movement = get_movement_amount(direction, distance);
    let new_lock_position = lock_position + movement;

    let zeros_crossed = if movement > 0 {
        // euclidian division rounds down towards negative infinity
        // for moving to the right, we want to count how many 100s we crossed. easy version
        new_lock_position.div_euclid(100)
    } else {
        // count euclidian multiples of 100 crossed when moving left
        (lock_position - 1).div_euclid(100) - (new_lock_position - 1).div_euclid(100)
    };
    total_zeros += zeros_crossed.abs();

    let lock_position = new_lock_position.rem_euclid(100);
    println!("lock position: {}", lock_position);
    (lock_position, total_zeros)
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
