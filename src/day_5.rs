use std::fs::File;
use std::io::{BufReader, BufRead};

enum StringError {
    NoAlternatingRepeats,
    NoPairRepeats
}

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

fn naughty_strings2() {
    
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
            "xy" | "cd" | "pq" | "ab" => bad_string = true,
            _ => ()
        }
        
        prev = c;
    }
    (vowels, double, bad_string)
}

// fn check_pairs(s: &mut String) -> Result<String, StringError> {
//     // Use a String slice to get a 2 letter pattern.
//     while s.len() > 3 {
//         for (c1, c2) in s.chars().enumerate() {
            
//         }
//     }
//     Ok(String::from("temp"))
// }

fn check_str_pairs(s: &mut str) {
    
}

fn check_repeats(s: String) -> Result<String, StringError> {
    // Check values from two indexes ago to ensure that the values repeat properly.
    
    Ok(String::from("temp"))
}