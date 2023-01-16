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
    file.read_to_string(&mut directions).unwrap();
    
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut current_pos: (i32, i32) = (0, 0);
    let mut robo_santa: (i32, i32) = (0, 0);
    let mut total_houses: i32 = 1;
    let mut santa_turn = true;
    
    map.insert((0, 0), 2);
    
    for c in directions.chars() {
        match c {
            '>' => {
                // println!("Going east...");
                if santa_turn {
                    current_pos.0 += 1;
                } else {
                    robo_santa.0 += 1;
                }
            },
            '<' => {
                // println!("Going west...");
                if santa_turn {
                    current_pos.0 -= 1;
                } else {
                    robo_santa.0 -= 1;
                }
            },
            '^' => {
                // println!("Going north...");
                if santa_turn {
                    current_pos.1 += 1;
                } else {
                    robo_santa.1 += 1;
                }
            },
            'v' => {
                // println!("Going south...");
                if santa_turn {
                    current_pos.1 -= 1;
                } else {
                    robo_santa.1 -= 1;
                }
            },
            err => panic!("Error in input: '{}' is not a valid direction!", err)
        }
        
        // Get entry from map
        if santa_turn {
            if map.contains_key(&current_pos) {
                map.entry(current_pos).and_modify(|value| {
                    *value += 1;
                    // println!("House @ ({}, {}) has {} presents.", current_pos.0, current_pos.1, value);
                });
            } else {
                // println!("Santa's house at ({}, {}) does not exist.", current_pos.0, current_pos.1);
                total_houses += 1;
                map.insert(current_pos, 1);
            }
        } else {
            if map.contains_key(&robo_santa) {
                map.entry(robo_santa).and_modify(|value| {
                    *value += 1;
                });
            } else {
                // println!("Robo Santa's house at ({}, {}) does not exist.", robo_santa.0, robo_santa.1);
                total_houses += 1;
                map.insert(robo_santa, 1);
            }
        }
        
        santa_turn = !santa_turn;
    }
    println!("Number of houses with presents: {}", total_houses);
}