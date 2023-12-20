use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
   let f = File::open(concat!(env!("CARGO_MANIFEST_DIR"),"\\input.txt"))?;
   let f = BufReader::new(f);
   let mut counter:u32 = 0;
   let mut counter2:u32 = 0;
   let mut previous_value: u32 = u32::MAX;
   let mut previous_values: [u32; 3] = [0; 3];
   
   for line in f.lines() {
      let value:u32= line?.parse().expect("Not a number!");
      if value > previous_value
      {
        counter += 1;
      }
      previous_value = value;

      let previous_sum: u32 = previous_values.iter().sum();
      let lost_value: u32 = previous_values[0];
      previous_values[0] = previous_values[1];
      previous_values[1] = previous_values[2];
      previous_values[2] = value;
   
      if lost_value > 0
      {
        let current_sum: u32 = previous_values.iter().sum::<u32>();
        print!("Previous sum: {} - Current sum: {}", previous_sum, current_sum );

        if current_sum > previous_sum
        {
            println!("(Increased)");
            counter2 += 1;
         }
         else { println!("(no change/decreased)"); }
        }
    }

   println!("Output Part 1: {}",counter);
   println!("Output Part 2: {}", counter2);

   Ok(())
}
