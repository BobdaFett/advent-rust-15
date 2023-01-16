//
// Advent of Code in Rust - created by Lucas Vas on 01/08/2022
//

use std::io::{stdin, stdout, Write};

pub mod day_1;
pub mod day_2;
pub mod day_3;

fn main() {
    let mut user_in = String::new();
    print!("Enter day to run: ");
    let _ = stdout().flush();
    stdin().read_line(&mut user_in).unwrap();

    if let Some('\n')=user_in.chars().next_back() {
        user_in.pop();
    }
    if let Some('\r')=user_in.chars().next_back() {
        user_in.pop();
    }
    
    match user_in.parse::<i32>().unwrap() {
        1 => day_1::floor_directions(),
        2 => day_2::wrapping_paper(),
        3 => day_3::directions(),
        other => panic!("Value {} is not a valid/developed day.", other)
    }
}
