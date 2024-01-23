use std::{cmp::{max, min}, collections::HashSet, fs::File, io::{BufReader, BufRead}, time::Instant};

fn main() {
    let now = Instant::now();
    
    let f = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "\\input.txt")).unwrap();
    let f = BufReader::new(f);

    let mut part1: u32 = 0;
    //let mut part2: u32 = 0;
    let mut game_board:[[i16; 10]; 10] = [[0;10];10];
    
    let mut i = 0;
    for line in f.lines() {
        let l = line.unwrap();
        let mut j = 0;
        for ch in l.as_bytes() {
            game_board[i][j] = (ch - b'0') as i16;
            j += 1;
        }
        i += 1;
    }

    for count in 0.. {
        let mut flashed: HashSet<(u8,u8)> = HashSet::new();
        for i in 0..10 {
            for j in 0..10 {
                game_board[i][j] += 1;
                if game_board[i][j] > 9 {
                    flash(i as i8, j as i8, &mut game_board, &mut flashed);
                }
            }
        }

        for i in 0..10 {
            for j in 0..10 {
                if game_board[i][j] > 9 {
                    game_board[i][j] = 0;
                }
            }
        }

        if count < 100 {
            part1 += flashed.len() as u32;
        }
        if flashed.len() == 100 {
            println!("Part2 : {}", count + 1);
            break;    
        }
        //println!("Part1 {}: {}", _count, part1);
    }
    
    println!("Part1 : {}", part1);
    //println!("Part2 : {}", part2);
    println!("Elapsed Time: {:.2?}", now.elapsed());

}

fn flash(i: i8, j: i8, game_board: &mut [[i16; 10]; 10], flashed: &mut HashSet<(u8,u8)>)
{
    if flashed.insert((i as u8,j as u8))
    {
        let sx:i8 = max(0, i - 1);
        let ex:i8 = min(10, i + 2);
        let sy:i8 = max(0, j - 1);
        let ey:i8 = min(10, j + 2);
        for x in sx..ex {
            for y in sy..ey {
                game_board[x as usize][y as usize] += 1;
                if game_board[x as usize][y as usize] > 9 {
                    flash(x,y, game_board, flashed);
                }
            }
        }
    }
}