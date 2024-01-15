use std::{time::Instant, fs::File, io::{BufReader, Read}, collections::HashMap};
    
fn main() {
    let now = Instant::now();
    
    let f = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "\\input.txt")).unwrap();
    let mut f = BufReader::new(f);

    let mut s = String::new();
    let _ = f.read_to_string(&mut s);

    let v = s.split(",")
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();

    let mut cache = HashMap::new();

    let part1: u64 = v.iter()
                .map(|n| calculate_created(80 - *n, &mut cache))
                .fold(0, |b, j | b + j)
                + v.len() as u64;

    let part2 = v.iter()
                .map(|n| calculate_created(256 - *n, &mut cache))
                .fold(0, |b, j | b + j)
                + v.len() as u64;

    println!("Part1 : {}", part1);
    println!("Part2 : {}", part2);
    println!("Elapsed Time: {:.2?}", now.elapsed());
}

fn calculate_created(remaining_steps: u32, cache: &mut HashMap<u32, u64>) -> u64
{
    if cache.contains_key(&remaining_steps)
    {
        return *cache.get(&remaining_steps).unwrap();
    }
    
    if remaining_steps <= 7 { 
        cache.insert(remaining_steps, 1);
        return 1; 
    }
    if remaining_steps <= 9 { 
        let n = calculate_created(remaining_steps - 7, cache) + 1; 
        cache.insert(remaining_steps, n);
        return n;
    }
    
    let n = calculate_created(remaining_steps - 7, cache) + calculate_created(remaining_steps - 9, cache) + 1;
    cache.insert(remaining_steps, n);
    return n;
} 
