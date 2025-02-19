use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, time::Instant};

fn main() {
    let now = Instant::now();
    
    let f = File::open(concat!(env!("CARGO_MANIFEST_DIR"),"\\input.txt")).unwrap();
    let f = BufReader::new(f);

    let mut mapping_map: HashMap<(char,char),char> = HashMap::new();
    let mut polymer_map: HashMap<(char,char),u64> = HashMap::new();
    
    let mut in_mappings = false;
    let mut last = ' ';
    
    for line in f.lines() {
        let l = line.unwrap();

        if l == "" {
            in_mappings = true;
            continue;
        }

        if in_mappings {
           let parts = l.split_once(" -> ").unwrap();
           mapping_map.insert((parts.0.chars().nth(0).unwrap(),parts.0.chars().nth(1).unwrap()), parts.1.chars().nth(0).unwrap());
        }
        else {
            // split the string into pairs of characters
            let mut prev = ' ';
            for c in l.chars() {
                if prev != ' ' {
                    let k = (prev, c);
                    let count = polymer_map.entry(k).or_insert(0);
                    *count += 1;
                }
                prev = c;
                last = c;
            }
        }
    }

    let mut starting_map = polymer_map.clone();
    let (min, max) = expand(&mut starting_map, &mapping_map, 10, &last);

    println!("Part 1: {}", max - min);

    let mut starting_map = polymer_map.clone();
    let (min, max) = expand(&mut starting_map, &mapping_map, 40, &last);

    println!("Part 2: {}", max - min);

    println!("Elapsed Time: {:.2?}", now.elapsed());

}

fn expand(polymer_map: &mut HashMap<(char,char),u64>, mapping_map: &std::collections::HashMap<(char, char), char>, loops: i32, last: &char) -> (u64, u64) {
    for _lp in 0..loops
    {
        // Loop through the map, creating a new map which contains the expanded protein
        let mut new_map = HashMap::new();
        for c in polymer_map.iter() {
            if mapping_map.contains_key(&c.0)  {
                let k1 = (c.0.0, *mapping_map.get(&c.0).unwrap());
                let count = new_map.entry(k1).or_insert(0);
                *count += c.1;
                let k2 = (*mapping_map.get(&c.0).unwrap(), c.0.1);
                let count = new_map.entry(k2).or_insert(0);
                *count += c.1;
            }
            else {
                let count = new_map.entry(*c.0).or_insert(0);
                *count += c.1;
            }
        }
        *polymer_map = new_map;

    }

    // Find the most and least common elements in the polymer
    let mut counts = std::collections::HashMap::new(); 
    for c in polymer_map.iter() {
        let count = counts.entry(c.0.0).or_insert(0);
        *count += c.1;
    }
    let t = counts.entry(*last).or_insert(0);
    *t += 1;

    let max: u64 = *counts.values().max().unwrap();
    let min: u64 = *counts.values().min().unwrap();

    return (min, max);
    
}
