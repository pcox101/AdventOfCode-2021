use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}, time::Instant};

fn main() {
    let now = Instant::now();
    
    let f = File::open(concat!(env!("CARGO_MANIFEST_DIR"),"\\input.txt")).unwrap();
    let f = BufReader::new(f);

    let mut hs = HashSet::new();
    
    let mut in_fold = false;
    let mut part1 = 0;
    
    for line in f.lines() {
        let l = line.unwrap();
        
        if l == "" {
            in_fold = true;
            continue;
        }

        if !in_fold {
            let t = l.split_once(",").unwrap();
            hs.insert((t.0.parse::<u32>().unwrap(),t.1.parse::<u32>().unwrap()));
        }
        else {
            if l.contains("y=") {
                let row:u32=l.split_once("=").unwrap().1.parse().unwrap();
                let iter2 = hs.into_iter().map(|v| { if v.1 > row { (v.0, row - (v.1 - row)) } else { (v.0,  v.1) } } );
                hs = iter2.collect();
            }
            else {
                let column=l.split_once("=").unwrap().1.parse().unwrap();
                hs = hs.into_iter().map(|v| { if v.0 > column { (column - (v.0 - column), v.1 ) } else { (v.0,  v.1) } } ).collect();
            }
        }

        if in_fold && (part1 == 0) {
            part1 = hs.len();
        }
    }

    println!("Output Part 1: {}",part1);

    // We have finished all the folding, now output it
    // get the maxium row and column
    let max_row = hs.iter().max_by(|a,b| a.1.cmp(&b.1)).unwrap().1;
    let max_column = hs.iter().max_by(|a,b| a.0.cmp(&b.0)).unwrap().0;
    
    // Output the grid
    for row in 0..=max_row {
        for column in 0..=max_column {
            if hs.contains(&(column,row)) {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!();
    }
    
    println!("Elapsed Time: {:.2?}", now.elapsed());
}
