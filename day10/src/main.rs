use std::{time::Instant, fs::File, io::{BufReader, BufRead}};

fn main() {
    let now = Instant::now();
    
    let f = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "\\input.txt")).unwrap();
    let f = BufReader::new(f);

    let mut part1: u32 = 0;
    let mut vp2: Vec<u64> = Vec::new();
    
    for line in f.lines() {
        let mut stack: Vec<&u8> = Vec::new(); 
        let l = line.unwrap();
        let mut corrupted = false;
        for ch in l.as_bytes() {
            match ch {
                b'('=>stack.push(ch),
                b'['=>stack.push(ch),
                b'{'=>stack.push(ch),
                b'<'=>stack.push(ch),
                b')'=>if *stack.pop().unwrap() != b'(' { part1 += 3; corrupted = true; break; },
                b']'=>if *stack.pop().unwrap() != b'[' { part1 += 57; corrupted = true; break; },
                b'}'=>if *stack.pop().unwrap() != b'{' { part1 += 1197; corrupted = true; break; },
                b'>'=>if *stack.pop().unwrap() != b'<' { part1 += 25137; corrupted = true; break; },
                _=>{println!("Invalid character!")}
            }
        }

        if !corrupted {

            let mut p2: u64 = 0;
            while let Some(ch) = stack.pop() {
                match ch {
                    b'('=>{ p2 *= 5; p2 += 1; },
                    b'['=>{ p2 *= 5; p2 += 2; },
                    b'{'=>{ p2 *= 5; p2 += 3; },
                    b'<'=>{ p2 *= 5; p2 += 4; },
                    _=>{println!("Invalid character!")}
                }
            }
            //println!("Adding : {}", p2);
            vp2.push(p2);
            }
    }

    vp2.sort_unstable();
    let part2: u64 = *vp2.get(vp2.len() / 2 as usize).unwrap();

    println!("Part1 : {}", part1);
    println!("Part2 : {}", part2);
    println!("Elapsed Time: {:.2?}", now.elapsed());

}
