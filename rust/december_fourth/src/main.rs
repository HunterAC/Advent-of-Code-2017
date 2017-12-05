use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;

fn main() {
    let path = Path::new("input");
    let display = path.display();

    let file = match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let mut valid: bool = true;
    let mut total: u32 = 0;

    for line in reader.lines() {
        let mut vec = Vec::new();
        let pass_phrase = match line {
            Ok(line) => line,
            Err(_) => panic!("Error getting line from BufReader"),
        };
        let words = pass_phrase.split_whitespace();
        for word in words {
            //Use .to_string() to convert the &str to a string that is owned by the heap
            //and won't go out of scope when the iterator moves to the next line
            vec.push(word.to_string());       
        }

        'outer: for i in 0..vec.len() {
            'inner: for j in i+1..vec.len() {
                if str::eq(&vec[i], &vec[j]) {
                    valid = false;
                    //Using labels to break outer loop when two words match
                    //This avoids checking the rest of the line
                    break 'outer;
                }
            }
        }
        if valid {
            total = total + 1;
        }
        valid = true;
    }
    println!("Total Valid: {}", total);
}
