use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let f = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "\\input.txt")).unwrap();
    let f = BufReader::new(f);

    let lines: Vec<u32> = f
                            .lines()
                            .map(|n| u32::from_str_radix(&n.unwrap(), 2).unwrap())
                            .collect();

    do_part1(&lines);

    do_part2(lines);
}


fn do_part2(lines: Vec<u32>) {
    let two: u32 = 2;

    let mut ox_gen: u32 = 0;
    let mut co2_scrub: u32 = 0;

    let mut bitmask: u32 = 0;

    for i in (0..12).rev() {
        let mut ones_ox: i32= 0;
        let mut zeros_ox: i32 = 0;

        let mut ones_co: i32= 0;
        let mut zeros_co: i32 = 0;

        let nextbit: u32 = two.pow(i);
        
        for x in &lines {
            
            if x & bitmask == ox_gen & bitmask {
                let bitmap: u32 = x & nextbit;
                if  bitmap > 0 { ones_ox += 1; } else { zeros_ox += 1; }  
            }

            if x & bitmask == co2_scrub & bitmask {
                let bitmap: u32 = x & nextbit;
                if  bitmap > 0 { ones_co += 1; } else { zeros_co += 1; }     
            }

        }

        // Most common
        if ones_ox >= zeros_ox
        {
            ox_gen += nextbit;
        }

        // Only one left case
        if ones_co + zeros_co == 1
        {
            if ones_co == 1 {
                co2_scrub += nextbit;
            }
        }
        // No ones, only zeros, always choose zero
        else if ones_co == 0
        {
            // nothing to do
        }
        // Least common
        else if ones_co < zeros_co
        {
            co2_scrub += nextbit;
        }
        bitmask += nextbit;
    }

    println!("Part 2: {} * {} = {}", co2_scrub, ox_gen, co2_scrub * ox_gen);
}



fn do_part1(lines: &Vec<u32>) {
    let mut g: u32 = 0;
    let mut e: u32 = 0;
    let two: u32 = 2;

    for i in 0..12 {
        let mut number_of_ones = 0;
        let mut number_of_zeros = 0;
        let bitmask: u32 = two.pow(i);
        for x in lines {
            let bitmap: u32 = x & bitmask;
            if bitmap == 0 {
                number_of_zeros += 1;
            }
            else {
                number_of_ones += 1;
            }
        }

        if number_of_ones > 0 {
            if number_of_ones > number_of_zeros {
                g += bitmask;
            }
            else {
                e += bitmask;
            }
        }
    }
    
    println!("Part 1: {} * {} = {}", g, e, g * e);
}
