use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Clone, Copy)]
struct Box {
    // stores the information needed for a box.
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
        let smallest: i32;
        if self.l < self.w {
            if self.l < self.h {
                smallest = self.l;
            } else {
                smallest = self.h;
            }
        } else {
            if self.w < self.h {
                smallest = self.w;
            } else {
                smallest = self.h;
            }
        }
        smallest
    }
    
    fn get_surface_area(self) -> i32 {
        (2 * self.l * self.w) + (2 * self.w * self.h) + (2 * self.h * self.l)
    }
    
    // fn get_smallest_side(self) -> Side {
    //     let smallest: Side;
    //     if self.a.get_area() < self.b.get_area() {
    //         if self.a.get_area() < self.c .get_area(){
    //             smallest = self.a;
    //         } else {
    //             smallest = self.c;
    //         }
    //     } else {
    //         if self.b.get_area() < self.c.get_area() {
    //             smallest = self.b;
    //         } else {
    //             smallest = self.c;
    //         }
    //     }
    //     smallest
    // }
    
    // fn get_area(self) -> i32 {
    //     self.a.get_area() * self.b.get_area() * self.c.get_area()
    // }
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
        
        total_area += current_box.get_surface_area(); // plus smallest size
        
        // total_ribbon += current_box.get_smallest_side().get_area() + current_box.get_area();
        
    }
    
    println!("Total paper needed: {} sq. ft.", total_area);
    println!("Total ribbon needed: {}", total_ribbon);
}