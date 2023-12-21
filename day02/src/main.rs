use std::{fs::File, io::{BufReader, BufRead}};
use regex::Regex;

fn main() {
    let f = File::open(concat!(env!("CARGO_MANIFEST_DIR"),"\\input.txt")).unwrap();
    let f = BufReader::new(f);
    
    let re = Regex::new("^(forward|up|down) (\\d*)$").unwrap();
    
    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;
    let mut depth2 = 0;

    for line in f.lines() {
        let hay = line.unwrap();
        let captures = re.captures(&hay).unwrap();
        let num: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
        
        if captures.get(1).unwrap().as_str() == "forward"
        {
            forward += num;
            depth2 += aim * num;
        }
        else if captures.get(1).unwrap().as_str() == "up"
        {
            depth -= num;
            aim -= num;
        }
        else if captures.get(1).unwrap().as_str() == "down"
        {
            depth += num;
            aim += num;
        }
    }

    println!("Output Part 1: {}",depth * forward);
    println!("Output Part 2: {}",depth2 * forward);
}  
