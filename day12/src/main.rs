use std::{borrow::Borrow, collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, time::Instant};

fn main() {
    let now = Instant::now();
    
    let f = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "\\input.txt")).unwrap();
    let f = BufReader::new(f);

    let mut hm = HashMap::new();
    
    for line in f.lines() {
        let l = line.unwrap();
        let s = l.split_once("-").unwrap();

        // Double link
        link(&mut hm, s.0.to_string() , s.1.to_string());
        link(&mut hm, s.1.to_string() , s.0.to_string());
    }

    let mut part1 = 0;
 
    // BFS through
    {
        let mut q = Vec::new();
        q.push(("start", HashSet::new()));
        
        while !q.is_empty()
        {
            let e = q.pop().unwrap();

            if e.0 == "end" {
                part1 += 1;
            }
            else {
                for n in hm.get(e.0).unwrap().iter() {
                    let mut s = e.1.clone();
                    if e.0.to_lowercase() == e.0 {
                        s.insert(e.0);
                    }
                    if !s.contains(&n.borrow()) {
                        q.push((n, s))
                    }
                }
            }
        }
    }

    println!("Part1 : {}", part1);

    let mut part2 = 0;

    // BFS through again, but this time holding a set of
    // visited locations and how many times they've been visited
    if true {
        let mut q = Vec::new();
        let mut visited = HashMap::new();
        visited.insert("start",1);
        q.push(("start", visited));
        
        while !q.is_empty()
        {
            let e = q.pop().unwrap();

            if e.0 == "end" {
                //println!("{:?}",e.1);
                part2 += 1;
            }
            else {
                for n in hm.get(e.0).unwrap().iter() {
                    if n == "start" {
                        continue;
                    }
                    let mut s = e.1.clone();
                    if n.to_uppercase() == n.as_str() {
                        //s.insert(n,1);
                        q.push((n, s))
                    }
                    else
                    {
                        if !s.contains_key(&n.borrow()) {
                            s.insert(n, 1);
                            q.push((n, s))
                        }
                        else
                        {
                            // Is there *anything* with a count greater than one?
                            if s.iter().filter(|&v|  {v.1 > &1} ).count() == 0 {
                                s.insert(n,2);
                                q.push((n, s))
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Part2 : {}", part2);
    println!("Elapsed Time: {:.2?}", now.elapsed());

}

fn link(hm: &mut HashMap<String, Vec<String>>, s1: String, s2: String)
{
    if hm.contains_key(&s1) {
        let v = hm.get_mut(&s1).unwrap();
        v.push(s2.to_string());
    }
    else {
        let mut v = Vec::new();
        v.push(s2.to_string());
        hm.insert(s1.to_string(), v);
    }
}