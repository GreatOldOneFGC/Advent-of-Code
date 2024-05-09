use std::fs;

#[derive(PartialEq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

enum Turn {
    Santa,
    RoboSanta,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found");
    let mut santa_position = Position { x: 0, y: 0};
    let mut previous_positions: Vec<Position> = Vec::new();
    let mut delivered_houses = 0;

    for character in input.chars() {
        deliver_presents(&mut delivered_houses, &mut santa_position, &mut previous_positions, character)
    }

    println!("Solution 1: {} houses received at least one present.", delivered_houses);

    santa_position = Position { x: 0, y: 0 };
    delivered_houses = 0;
    previous_positions.clear();
    let mut robo_santa_position = Position { x: 0, y: 0};
    let mut turn = Turn::Santa;

    for character in input.chars() {
        match turn {
            Turn::Santa => {
                deliver_presents(&mut delivered_houses, &mut santa_position, &mut previous_positions, character);
                turn = Turn::RoboSanta;},
            Turn::RoboSanta => {
                deliver_presents(&mut delivered_houses, &mut robo_santa_position, &mut previous_positions, character);
                turn = Turn::Santa;},
        }
    }

    println!("Solution 2: {} houses received at least one present.", delivered_houses);
}

fn deliver_presents(delivered_houses: &mut i32, current_position: &mut Position, previous_positions: &mut Vec<Position>, character: char) {
    if !previous_positions.contains(&current_position) {
        *delivered_houses += 1;
    }
    previous_positions.push(current_position.clone());

    match character {
        '^' => current_position.y -= 1,
        'v' => current_position.y += 1,
        '<' => current_position.x -= 1,
        '>' => current_position.x += 1,
        _ => (),
    }
}