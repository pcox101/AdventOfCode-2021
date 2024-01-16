use std::{time::Instant, fs::File, io::{BufReader, Read}};

fn main() {
    let now = Instant::now();
    
    let f = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "\\input.txt")).unwrap();
    let mut f = BufReader::new(f);

    let mut s = String::new();
    let _ = f.read_to_string(&mut s);

    let mut v = s.split(",")
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();

    v.sort_unstable();

    let median = v.get(v.len() / 2 - 1).unwrap();   
                        
    let part1: u32 = v.iter()
                .fold(0, |b, j | b + j.abs_diff(*median));

    let mut part2 = i32::MAX;
    for suggestion in *v.get(0).unwrap()..*v.get(v.len() - 1).unwrap() + 1 {
        part2 = core::cmp::min(part2, v.iter()
                            .map(|entry| {let a = suggestion.abs_diff(*entry); (a * (1 + a)) / 2 })
                            .fold(0, |b, j| b + j as i32 ));
    }

    println!("Part1 : {}", part1);
    println!("Part2 : {}", part2);
    println!("Elapsed Time: {:.2?}", now.elapsed());
}
