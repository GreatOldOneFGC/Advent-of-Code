use std::{cmp::min, fs};

struct Present {
    length: i32,
    width: i32,
    height: i32,
}

impl Present {
    fn surface_area(&self) -> i32 {
        let first_side = self.length * self.width;
        let second_side = self.width * self.height;
        let third_side = self.height * self.length;
        let sides = vec![first_side, second_side, third_side];
        let smallest_side = sides.iter().min().unwrap();

        (first_side * 2) + (second_side * 2) + (third_side * 2) + smallest_side
    }

    fn ribbon(&self) -> i32 {
        let mut dimensions = vec![self.length, self.width, self.height];
        dimensions.sort();
        let ribbon = (dimensions[0] * 2) + (dimensions[1] * 2);
        
        let bow = self.length * self.width * self.height;
        ribbon + bow
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found!");

    let mut total_area = 0;
    let mut total_ribbon = 0;

    for line in input.lines() {
        let raw_values: Vec<&str> = line.split('x').collect();
        let mut values = vec![];
        for value in raw_values {
            values.push(value.parse::<i32>().unwrap());
        }

        let present = Present { length: values[0], width: values[1], height: values[2]};
        total_area += present.surface_area();
        total_ribbon += present.ribbon();
    }

    println!("Solution 1: The elves should order {} square feet of wrapping paper.", total_area);
    println!("Solution 2: The elves should order {} feet of ribbon.", total_ribbon);
}