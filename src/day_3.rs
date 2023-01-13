use std::fs::File;
use std::io::Read;
use std::collections::HashMap;


pub fn directions() {
    let mut file: File = if let Ok(file) = File::open("./txt/day_3.txt") {
        file
    } else {
        panic!("Could not open file!");
    };
    
    let mut directions: String = String::new();
    match file.read_to_string(&mut directions) {
        Ok(_) => println!("File read successfully."),
        Err(e) => panic!("Error: {:?}", e)
    }
    
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut current_pos: (i32, i32) = (0, 0);
    let mut houses_with_extra: i32 = 0;
    
    for c in directions.chars() {
        match c {
            '>' => {
                println!("Going east...");
                current_pos.0 += 1;
            },
            '<' => {
                println!("Going west...");
                current_pos.0 -= 1;
            },
            '^' => {
                println!("Going north...");
                current_pos.1 += 1;
            },
            'v' => {
                println!("Going south...");
                current_pos.1 -= 1;
            },
            err => panic!("Error in input: {} is not a valid direction!", err)
        }
        
        // Get entry from map
        map.entry(current_pos).and_modify(|value| {
            *value += 1;
            println!("House @ ({}, {}) has {} presents.", current_pos.0, current_pos.1, value);
            if *value > 1 {
                println!("Adding one to var");
                houses_with_extra += 1;
            }
        }).or_insert(1);
    }
    
    println!("Number of houses with duplicate presents: {}", houses_with_extra);
}