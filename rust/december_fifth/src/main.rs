use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;

fn main() {
    let path = Path::new("input");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't read {}: {}", path.display(), why),
    };
    let reader = BufReader::new(file);

    let mut vec: Vec<i32> = Vec::new();
    
    for line in reader.lines() {
        let step_str = match line {
            Ok(line) => line,
            Err(_) => panic!("Error getting line from BufReader"),
        };

        let step_str = step_str.trim();
        
        match step_str.parse::<i32>() {
            Ok(n) => vec.push(n),
            Err(_) => panic!("Error parsing int from string"),
        };
    }

    //Creating copy of vec for part 2
    let mut vec_part_two: Vec<i32> = Vec::new();
    for i in 0..vec.len() {
        vec_part_two.push(vec[i]);
    }

    let mut steps = 0;
    let mut place: i32 = 0; 
    let mut old_place: i32 = 0;
    let end: i32 = vec.len() as i32;
    while place < end {
        old_place = place;
        place += &vec[place as usize];
        vec[old_place as usize] += 1;
        steps += 1;
    }

    println!("Part 1 took {} steps", steps);

    steps = 0;
    place = 0;
    old_place = 0;
    while place < end {
        old_place = place;
        place += &vec_part_two[place as usize];
        if vec_part_two[old_place as usize] >= 3 {
            vec_part_two[old_place as usize] -= 1;
        } else {
            vec_part_two[old_place as usize] += 1;
        }
        steps += 1;
    }
    println!("Part 2 took {} steps", steps);
}
