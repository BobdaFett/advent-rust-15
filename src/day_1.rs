use std::fs::File;
use std::io::prelude::*;

pub fn floor_directions() {
    let mut paren_file = match File::open("parens.txt") {
        Ok(file) => file,
        Err(_) => {
            match File::create("day_1.txt") {
                Ok(_) => panic!("Created file day_1.txt"),
                Err(e) => panic!("File creation failed: {:?}", e)
            };
        }
    };
    
    let mut file_buffer = String::new();
        match paren_file.read_to_string(&mut file_buffer) {
        Ok(_) => println!("File read successfully."),
        Err(e) => panic!("Error: {:?}", e)
    };
    
    // let as_string = file_buffer.to_string();
    // println!("The length of the string is {}", as_string.len());
    
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