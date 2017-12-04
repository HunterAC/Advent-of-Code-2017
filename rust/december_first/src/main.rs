use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::vec::Vec;

fn main() {
    let path = Path::new("input");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    println!("Is this for part one or two?");
    let mut problem = String::new();
    io::stdin().read_line(&mut problem).expect("Failed to read line");
    let problem = problem.trim();

    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).expect("Couldn't read file");
    buffer.trim();

    let chars: Vec<_> = buffer.chars().collect();
    let mut vec: Vec<u32> = Vec::new();

    for c in &chars {
        if c.to_digit(10).is_some() {
            vec.push(c.to_digit(10).unwrap());
        }
    }

    let mut total: u32 = 0;
    let mut index = 0;

    if problem == "one" {
        for i in 0..&vec.len() - 0 {
            if i == (&vec.len() - 1) {
                if &vec[i] == &vec[0] {
                    total = total + &vec[i];
                }
            }
            else if &vec[i] == &vec[i + 1] {
                total = total + &vec[i];
            }
        }
        println!("Total: {}", total);
    }
    else if problem == "two" {
        for i in 0..&vec.len() - 0 {
            index = (i + (&vec.len() / 2)) % &vec.len();
            if &vec[i] == &vec[index] {
                total = total + vec[i];
            }
        }

        println!("Total: {}", total);
    }
    else {
        println!("Enter one or two next time");
    }

}
