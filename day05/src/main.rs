use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};
use regex::Regex;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    
    let f = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "\\input.txt")).unwrap();
    let f = BufReader::new(f);

    let re = Regex::new(r#"(\d*),(\d*) -> (\d*),(\d*)"#).unwrap(); 

    let mut part1locations: HashMap<String,u32> = HashMap::new(); 
    let mut part2locations: HashMap<String,u32> = HashMap::new(); 

    for s in f.lines()
    {
        parse_line(s.unwrap(), &mut part1locations, &mut part2locations, &re);
    }

    let mut part1 = 0;
    for n in part1locations {
        if n.1 > 1 { part1 += 1; }
    }

    println!("Part1: {}",part1);

    let mut part2 = 0;
    for n in part2locations {
        if n.1 > 1 { part2 += 1; }
    }

    println!("Part2: {}",part2);

    println!("Elapsed Time: {:.2?}", now.elapsed());

}

fn parse_line(s: String, part1locations: &mut HashMap<String,u32>, part2locations: &mut HashMap<String,u32>, re: &Regex)
{
    let caps = re.captures(&s).unwrap();
    let x1 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let y1 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let x2 = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let y2 = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();

    let valid_for_part1 = (x1 == x2) || (y1 == y2);
    
    let mut x_offset = 0;
    let mut y_offset = 0;
    let mut x = x1;
    let mut y = y1;
    if x1 > x2 { x_offset = -1; } else if x1 < x2 { x_offset = 1; }
    if y1 > y2 { y_offset = -1; } else if y1 < y2 { y_offset = 1; }
    
    while (x - x_offset != x2) || (y - y_offset != y2)
    {
        let loc1 = format!("{},{}", x, y);
        let loc2 = format!("{},{}", x, y);
        
        if valid_for_part1 {
            let l = part1locations.get(&loc1);
            match l {
                None=>part1locations.insert(loc1, 1),
                Some(i)=>part1locations.insert(loc1, i + 1),
            };
        }
        let l = part2locations.get(&loc2);
        match l {
            None=>part2locations.insert(loc2, 1),
            Some(i)=>part2locations.insert(loc2, i + 1),
        };

        x += x_offset;
        y += y_offset;
    }
}
