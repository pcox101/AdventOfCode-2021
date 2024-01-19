use std::{time::Instant, fs::File, io::{BufReader, BufRead}, collections::HashSet};

fn main() {
    let now = Instant::now();
    
    let f = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "\\input.txt")).unwrap();
    let f = BufReader::new(f);

    let mut part1 = 0;
    let mut part2 = 0;

    for l in f.lines() {
        let s = l.unwrap();
        let vs = s.split("|").collect::<Vec<&str>>();
        
        let vp0 = vs.get(0).unwrap()
                                .split(" ")
                                .filter(|n| !n.is_empty())
                                .map(|n| n.to_string())
                                .collect::<Vec<String>>();

        let vp1 = vs.get(1).unwrap()
                .split(" ")
                .filter(|n| !n.is_empty())
                .map(|n| n.to_string())
                .collect::<Vec<String>>();

        part1 += vp1
                .iter()
                .fold(0, |b, j| if j.len() == 2 || j.len() == 3 || j.len() == 4 || j.len() == 7 { b + 1 } else { b });

        part2 += calc_part2(vp0, vp1);
        
    }

    println!("Part1 : {}", part1);
    println!("Part2 : {}", part2);
    println!("Elapsed Time: {:.2?}", now.elapsed());
}

fn calc_part2(vp0: Vec<String>, vp1: Vec<String>) -> i32 {
    let mut numbers: Vec<String> = vec![String::new(); 10];
    for s in vp0.iter() {
        let l = s.len();
        if l == 2 {
            numbers[1] = s.to_string();
        }
        else if l == 3 {
            numbers[7] = s.to_string();
        }
        else if l == 4 {
            numbers[4] = s.to_string();
        }
        else if l == 7 {
            numbers[8] = s.to_string();
        }
    }

    // 0, 6, 9 have 6 characters
    let mut six_chars: Vec<String> = vp0.iter()
                        .filter(|n| n.len() == 6)
                        .map(|n| (*n).to_string())
                        .collect::<Vec<String>>();

    // 9 contains 4 completely
    numbers[9] = get_containing(&mut six_chars, &numbers[4]);
    // Then 0 contains 1 completely
    numbers[0] = get_containing(&mut six_chars, &numbers[1]);
    // 6 remains
    numbers[6] = six_chars[0].to_string();

    // 2, 3, 5 have 5 characters
    let mut five_chars: Vec<String> = vp0.iter()
            .filter(|n| n.len() == 5)
            .map(|n| (*n).to_string())
            .collect::<Vec<String>>();

    // 3 contains 1 completely
    numbers[3] = get_containing(&mut five_chars, &numbers[1]);
    // Then 6 contains 5 completely
    if a_contains_b(&numbers[6], &five_chars[0]) {
        numbers[5] = five_chars.swap_remove(0);
    }
    else {
        numbers[5] = five_chars.swap_remove(1);   
    }
    // 2 remains    
    numbers[2] = five_chars[0].to_string();
    
    // Now do the second part
    let mut result = 0;
    for second in vp1 {
        for i in 0..10 {
            if a_equals_b(&second, &numbers[i]) {
                result *= 10;
                result += i;
                break;
            }
        }
    }

    println!("Line : {}", result);
    return result as i32;

}

fn get_containing(set: &mut Vec<String>, s: &String) -> String {
    let mut found = 0;
    for i in 0..set.len()  {
        let v = set.get(i).unwrap();
        if a_contains_b(&v, &s) {
            found = i;
            break;
        }
    }
    
    return set.swap_remove(found);
}

fn a_contains_b(a: &String, b: &String) -> bool {
    let mut a_hs: HashSet<u8> = HashSet::new();
    a_hs.extend(a.to_string().into_bytes());
    
    let mut b_hs: HashSet<u8> = HashSet::new();
    b_hs.extend(b.to_string().into_bytes());
    
    return a_hs.is_superset(&b_hs);
}

fn a_equals_b(a: &String, b: &String) -> bool
{
    let mut a_hs: HashSet<u8> = HashSet::new();
    a_hs.extend(a.to_string().into_bytes());
    
    let mut b_hs: HashSet<u8> = HashSet::new();
    b_hs.extend(b.to_string().into_bytes());

    return a_hs == b_hs;
}