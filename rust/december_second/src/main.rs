use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;

fn main() {
    //Start of Part 1

    let path = Path::new("input");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Coud not open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut total = 0;
    let reader = BufReader::new(file);
    let mut largest = 0;
    let mut smallest = 0;

    for line in reader.lines() {
        let mut vec: Vec<u32> = Vec::new();
        let number_line = line.unwrap();
        let numbers = number_line.split_whitespace();//Could also use .split("\t") in this case
        for num in numbers {
            if num.parse::<u32>().is_ok() {
                vec.push(num.parse::<u32>().unwrap());
            }
        }

        largest = &vec[0]-0;
        smallest = &vec[0]-0;

        for i in 0..vec.len() {
            if &vec[i]-0 > largest {
                largest = &vec[i]-0;
            }
            else if &vec[i]-0 < smallest {
                smallest = &vec[i]-0;
            }
        }

        total = total + (largest - smallest);
    }
    println!("Part 1 Total: {}", total);

    //Start of Part 2

    //Had to redeclare the file input again for some reason, not sure about that yet
    let path = Path::new("input");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Coud not open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut total = 0;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let mut vec: Vec<u32> = Vec::new();
        let number_line = match line { //Safer version of using if .is_ok() with .unwrap()
            Ok(line) => line,
            Err(_) => panic!("Error getting line from BufReader"),
        };
        let numbers = number_line.split_whitespace();
        for num in numbers {
            match num.parse::<u32>() {
                Ok(n) => vec.push(n),
                Err(_) => panic!("Error parsing u32 from string"),
            }
        }

        for i in 0..vec.len() {
            for j in i+1..vec.len() {
                if &vec[i] % &vec[j] == 0 {
                    total = total + (&vec[i]/&vec[j]);
                }
                else if &vec[j] % &vec[i] == 0 {
                    total = total + (&vec[j]/&vec[i]);
                }
            }
        }
    }
    println!("Part 2 Total: {}", total);
}
