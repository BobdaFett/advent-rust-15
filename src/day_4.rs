use std::fs::File;
use std::io::{Read, Seek, SeekFrom};


// I think this is quite possibly the slowest imaginable way of doing this, with the MD5 hash taking its time 
// and having to iterate through every number available. I would highly recommend *never* using this function
// unless there's absolutely no other choice and/or you have a lot of time on your hands.
pub fn advent_coins() {
    let mut file = File::open("./txt/day_4.txt").unwrap();
    
    let mut matched = false;
    let mut current_number = 0;
    
    while !matched {
        // get the new md5 of a string with the number attached.
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        let new_md5 = md5::compute(s + &current_number.to_string());
        let mut temp = 0;
        // println!("{:?}", new_md5);
        for c in format!("{:?}", new_md5).chars() {
            if c == '0' {
                // println!("Found zero: {:?} with {}", new_md5, current_number);
                temp += 1;
                continue;
            }
            break;
        }
        if temp == 6 {
            matched = true;
        } else {
            file.seek(SeekFrom::Start(0)).unwrap();
            current_number += 1;
        }
    }
    println!("The number with all six is: {}", current_number);
}

pub fn get_md5(s: String) {
    println!("{:?}", md5::compute(s));
}