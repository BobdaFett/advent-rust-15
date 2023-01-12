use std::fs::File;
use std::io::Read;


pub fn directions() {
    let mut file = if let Ok(file) = File::open("./txt/day_3.txt") {
        file
    } else {
        panic!("Could not open file!");
    };
    
    let mut file_buffer = String::new();
    match file.read_to_string(&mut file_buffer) {
        Ok(_) => println!("File read successfully."),
        Err(e) => panic!("Error: {:?}", e)
    }
}