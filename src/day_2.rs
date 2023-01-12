use std::fs::File;
use std::io::{BufReader, BufRead};

// All of the data needed for a box.
#[derive(Clone, Copy)]
struct Box {
    l: i32,
    w: i32,
    h: i32
}

impl Box {
    fn new(l: i32, w: i32, h: i32) -> Box {
        Box {
            l,
            w,
            h
        }
    }
    
    fn get_volume(self) -> i32 {
        self.l * self.w * self.h
    }
    
    fn get_smallest_side(self) -> i32 {
        let a = self.l * self.w;
        let b = self.w * self.h;
        let c = self.h * self.l;
        let smallest: i32;
        // l -> a, w -> b, h -> c
        if a < b {
            if a < c {
                smallest = a;
            } else {
                smallest = c;
            }
        } else {
            if b < c {
                smallest = b;
            } else {
                smallest = c;
            }
        }
        smallest
    }
    
    fn get_surface_area(self) -> i32 {
        (2 * self.l * self.w) + (2 * self.w * self.h) + (2 * self.h * self.l)
    }
}

pub fn wrapping_paper() {
    
    let mut total_area: i32 = 0;
    let mut total_ribbon: i32 = 0;
    
    // Read from file here
    let file = match File::open("day_2.txt") {
        Ok(file) => BufReader::new(file).lines(),
        Err(e) => panic!("Error: {:?}", e)
    };
    
    for line in file {
        let current_box: Box;
        match line {
            Ok(l) => {
                let temp = l.as_str().split('x').collect::<Vec<&str>>();
                
                // vars for readability and changeability
                let x = temp[0].parse().unwrap();
                let y = temp[1].parse().unwrap();
                let z = temp[2].parse().unwrap();
                
                current_box = Box::new(x, y, z);
            },
            Err(e) => panic!("Line could not be read: {:?}", e)
        }
        
        // total ribbon = perimeter of smallest face + total volume of box
        
        total_area += current_box.get_surface_area() + current_box.get_smallest_side();
        
        // total_ribbon += current_box.get_smallest_side().get_area() + current_box.get_area();
        
    }
    
    println!("Total paper needed: {} sq. ft.", total_area);
    println!("Total ribbon needed: {}", total_ribbon);
}