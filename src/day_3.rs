use std::fs::File;
use std::io::Read;

enum MapError {
    HouseDne
}

struct Coord {
    x: i32,
    y: i32
}

struct HouseMap {
    origin: Coord, // placeholder
    arr: Vec<Vec<i32>>
}

impl HouseMap {
    fn new() -> HouseMap {
        let temp_vec: Vec<Vec<i32>> = Vec::from(Vec::new());
        HouseMap {
            origin: Coord { x: 0, y: 0 },
            arr: temp_vec
        }
    }
    
    fn push(self, x: i32, y: i32) -> Result<(), MapError> {
        
        Ok(())
    }
}

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
    
    // Do Vectors expand automatically?
    
    
}