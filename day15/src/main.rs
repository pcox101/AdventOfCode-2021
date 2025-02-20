use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}, fs::File, io::{BufRead, BufReader}, time::Instant};

static DELTA: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() {
    let now = Instant::now();
    
    let f = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "\\input.txt")).unwrap();
    let f = BufReader::new(f);

    let game_board = f.lines()
                .map(|n| n.unwrap()
                    .as_bytes()
                    .iter()
                    .map(|s| s - '0' as u8)
                    .collect::<Vec<u8>>())
                .collect::<Vec<Vec<u8>>>();

    let part1 = dijkstra(&game_board);

    let mut new_game_board: Vec<Vec<u8>> = Vec::new();

    let height = game_board.len() as i32;
    let width = game_board[0].len() as i32;

    for row in 0..(height*5) {
        let mut newline = Vec::new();
        for col in 0..(width*5) {
            let mut newval = game_board[(row % height) as usize][(col % width) as usize];
            newval += (row / height + col / width) as u8;
            newval = ((newval - 1) % 9) + 1;
            newline.push(newval);
        }
        new_game_board.push(newline);
    }

    let part2 = dijkstra(&new_game_board);

    println!("Part1 : {}", part1);
    println!("Part2 : {}", part2);
println!("Elapsed Time: {:.2?}", now.elapsed());

}

fn dijkstra(game_board: &Vec<Vec<u8>>) -> u32 {
    let height = game_board.len() as i32;
    let width = game_board[0].len() as i32;

    let mut lowest_cost = 0;
    
    let mut q: BinaryHeap<Reverse<(u32, i32, i32)>> = BinaryHeap::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    q.push(Reverse((0,0,0)));
    
    while let Some(n) = q.pop()
    {
        let entry = n.0;
        //println!("Visiting ({}, {}, {}) ", entry.0, entry.1, entry.2);
    
        if entry.1 == height - 1 && entry.2 == width - 1
        {
            lowest_cost = entry.0;
            break;
        }
    
        for dxdy in DELTA.iter() {
            let nx = entry.1 + dxdy.0;
            let ny = entry.2 + dxdy.1;
    
            if nx < 0 || nx >= width as i32 || ny < 0 || ny >= height as i32 { continue; }
            if visited.contains(&(nx, ny)) { continue; }
    
            visited.insert((nx, ny));
            let nentry = Reverse((entry.0 + game_board[nx as usize][ny as usize] as u32,nx, ny));
            q.push(nentry);
    
        }
    }

    return lowest_cost;
    }
