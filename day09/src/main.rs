use std::{time::Instant, fs::File, io::{BufReader, BufRead}, collections::HashSet};

fn main() {
    let now = Instant::now();
    
    let f = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "\\input.txt")).unwrap();
    let f = BufReader::new(f);

    let mut part1: u32 = 0;
    let mut part2: u32 = 1;

    let game_board = f.lines()
                .map(|n| n.unwrap()
                    .as_bytes()
                    .iter()
                    .map(|s| s - '0' as u8)
                    .collect::<Vec<u8>>())
                .collect::<Vec<Vec<u8>>>();

    let mut vp2: Vec<u32> = Vec::new();

    for i in 0..game_board.len() {
        for j in 0..game_board[0].len() {
            let mut l = u8::MAX;
            let mut r = u8::MAX;
            let mut u = u8::MAX;
            let mut d = u8::MAX;

            if i != 0 { l = game_board[i - 1][j]; };
            if i != game_board.len() - 1 { r = game_board[i + 1][j]; };
            if j != 0 { u = game_board[i][j - 1]; };
            if j != game_board[0].len() - 1 { d = game_board[i][j + 1]; };

            let c = game_board[i][j];
            if (c < l) && (c < r) && (c < u) && (c < d) {
                part1 += c as u32 + 1;

                // Flood fill (literally!)
                let mut q: Vec<(usize,usize)> = Vec::new();
                let mut visited: HashSet<(usize, usize)> = HashSet::new();
                let mut counter = 0;
                
                q.push((i,j));

                while let Some(n) = q.pop()
                {
                    if game_board[n.0][n.1] != 9 && visited.insert((n.0, n.1))
                    {
                        counter += 1;
                        if n.0 != 0 { q.push((n.0 - 1, n.1)); };
                        if n.0 != game_board.len() - 1 { q.push((n.0 + 1, n.1)); };
                        if n.1 != 0 { q.push((n.0, n.1 - 1)); };
                        if n.1 != game_board[0].len() - 1  { q.push((n.0, n.1 + 1)) };
                    }
                }

                vp2.push(visited.len() as u32);
            }
        }
    }

    vp2.sort();
    vp2.reverse();

    for i in 0..3 {
        part2 *= vp2[i];
    }

    println!("Part1 : {}", part1);
    println!("Part2 : {}", part2);
    println!("Elapsed Time: {:.2?}", now.elapsed());

}
