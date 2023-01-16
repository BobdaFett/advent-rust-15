use std::fs::File;
use std::io::prelude::*;

pub fn floor_directions() {
    let mut paren_file = match File::open("./txt/day_1.txt") {
        Ok(file) => file,
        Err(e) => panic!("Error: {:?}", e)
    };
    
    let mut file_buffer = String::new();
    paren_file.read_to_string(&mut file_buffer).unwrap();
    
    let mut current_floor: i32 = 0;
    let mut been_in_basement: bool = false;
    let mut current_index = 1;
    
    for c in file_buffer.chars() {
        if c == '(' {
            current_floor += 1;
        }
        if c == ')' {
            current_floor -= 1;
        }
        if !been_in_basement && current_floor < 0 {
            been_in_basement = true;
            println!("The first time Santa is in the basement is at {}", current_index);
        }
        current_index += 1;
    }
    
    println!("The floor that Santa should be on is {}", current_floor);
}