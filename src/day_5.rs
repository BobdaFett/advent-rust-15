use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn naughty_strings() {
    let file = File::open("./txt/day_5.txt").unwrap();
    let buffer = BufReader::new(file).lines();
    let mut total_words = 0;
    
    for line in buffer {
        match line {
            Ok(v) => {
                let temp = check_string(v);
                
                if temp.0 {  // vowels
                    if temp.1 {  // duplicate letters
                        if !temp.2 {  // no bad strings
                            total_words += 1;
                        }
                    }
                }
            },
            Err(e) => println!("Error: {:?}", e)
        }
    }
    
    println!("Total nice words: {}", total_words);
    
}

fn check_string(s: String) -> (bool, bool, bool) {
    let mut vowels = false;
    let mut double = false;
    let mut bad_string = false;
    let mut prev: char = ' ';  // should be impossible to be a space
    let mut num_vowels = 0;
    
    for c in s.chars() {
        if prev == c {
            double = true;
        }
        
        if let 'a' | 'e' | 'i' | 'o' | 'u' = c { num_vowels += 1; }
        if num_vowels == 3 {
            vowels = true;
        }
        
        let temp_string = prev.to_string() + &c.to_string();
        match temp_string.as_str() {
            "xy" | "cd" | "pq" | "ab" => {
                bad_string = true;
            },
            _ => ()
        }
        
        prev = c;
    }
    (vowels, double, bad_string)
}