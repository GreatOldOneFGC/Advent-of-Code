use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found.");
    let mut floor = 0;
    
    for character in input.chars() {
        check_floor(&mut floor, character)
    }

    println!("Solution 1: Santa ends up on Floor {}", floor);

    floor = 0;
    let mut position = 0;

    for character in input.chars() {
        position += 1;
        check_floor(&mut floor, character);
        if floor < 0 {
            break
        };
    }
    
    println!("Solution 2: Santa enters the basement at Position {}", position);
}

fn check_floor(floor: &mut i32, character: char) {
    if character == '(' {
        *floor += 1;
    }
    else if character == ')' {
        *floor -=1;            
    }
}