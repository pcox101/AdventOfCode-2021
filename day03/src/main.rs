use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let f = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "\\input.txt")).unwrap();
    let f = BufReader::new(f);

    //let mut number_of_ones: [i32; 15] = [0; 15];
    //let mut number_of_zeros: [i32; 15] = [0; 15];
    
    let lines: Vec<u32> = f
                            .lines()
                            .map(|n| u32::from_str_radix(&n.unwrap(), 2).unwrap())
                            .collect();


    let mut g: u32 = 0;
    let mut e: u32 = 0;
    for i in 0..11 {
        for x in &lines {
            println!("{x}");
        }
    }
    
/*    for line in f.lines() {
        let buf = line.unwrap();
        let buf_r = buf.as_bytes();
        let mut counter = 0;
        for char in buf_r {
            if *char == 48 as u8 {
                number_of_zeros[counter] += 1;
            }
            else {
                number_of_ones[counter] += 1;
            }
            counter += 1;
            if counter > max_counter { max_counter = counter; }
        }
    }

    for i in 0..max_counter {
       g *= 2;
       e *= 2;
       if number_of_ones[i] > number_of_zeros[i] {
            g += 1;
       }
       else {
           e += 1;
       }
    }
    
    println!("Part 1: {} * {}", g, e);
     */
}
