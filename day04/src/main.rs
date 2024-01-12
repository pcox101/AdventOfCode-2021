use std::{fs::File, io::{BufReader, Read}, collections::{HashMap, HashSet}};

struct Board {
    id: u32,
    numbers_map: HashMap<u32, u32>,
    numbers_list: Vec<u32> 
}

fn main() {
    let f = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "\\input.txt")).unwrap();
    let mut f = BufReader::new(f);

    let mut s = String::new();
    let _ = f.read_to_string(&mut s);

    let (draws, boards) = s.split_once("\r\n\r\n").unwrap();
    
    let draws = draws
                   .split(",")
                   .map(|n| n.parse::<u32>().unwrap());

    let mut board_number = 0;
    let mut board_numbers: HashSet<u32> = HashSet::new();
    
    let mut boards = boards
                        .split("\r\n\r\n")
                        .map(|n| { board_numbers.insert(board_number); board_number += 1; build_board(n, board_number - 1) })
                        .collect::<Vec<Board>>();

    let mut part1 = 0;
    let mut part2 = 0;

    // Play the draws
    for d in draws
    {
        for b in boards.iter_mut()
        {
            if board_numbers.contains(&b.id) && b.numbers_map.contains_key(&d) {
                let i = b.numbers_map.get(&d).unwrap();
                b.numbers_list[*i as usize] = 99999;

                // See if this completes a row or a column
                let row_start = i - (i % 5);
                let mut row_total = 0;
                for j in row_start..row_start + 5
                {
                    row_total += b.numbers_list[j as usize];
                }
                
                let column_start = i % 5;
                let mut column_total = 0;
                for j in (column_start..column_start + 25).step_by(5) {
                    column_total += b.numbers_list[j as usize];
                }

                if (row_total == 5 * 99999) || (column_total == 5 * 99999) {
                    // Completed a board, let's look at it
                    let mut sum = 0;
                    for i in b.numbers_list.iter() {
                        if *i != 99999 { sum += i};
                    }

                    if part1 == 0 {
                        part1 = d * sum;
                    }   

                    // Is this the last one completed?
                    if board_numbers.len() == 1
                    {
                        part2 = d * sum;
                    }
                    
                    // Remove this board from future plays
                    board_numbers.remove(&b.id);
                }
            }
        }
    }
    println!("Output part 1 {}", part1);
    println!("Output part 2 {}", part2);

    
}

fn build_board(board_string: &str, board_number: u32) -> Board
{
    let board_string = str::replace(board_string, "\r\n"," ");
    
    let v = board_string
                    .split(" ")
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<u32>().unwrap());
                    
    let mut board_map = HashMap::new();
    let mut board_list = Vec::new();

    let mut i = 0;
    for n in v {
        board_map.insert(n, i);
        board_list.push(n);
        i += 1; 
    }
    
    let b = Board {
        id: board_number,
        numbers_map: board_map,
        numbers_list: board_list,
    };

    return b;

}
