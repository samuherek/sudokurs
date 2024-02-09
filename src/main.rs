mod sudoku;

use sudoku::{Sudoku, CellIdx};
use std::fs;
use std::path::PathBuf;
use rand::{thread_rng, Rng};

fn load_skds() {
    let mut output: Vec<Sudoku> = Vec::new();

    for _ in 0..1000 {
        let mut sdk = Sudoku::new(None);
        sdk.fill_grid(0);
        // sdk.print();
        output.push(sdk);
    }

    let value: String = output.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n");
    fs::write(PathBuf::from("output.txt"), value).unwrap();
}

fn is_unique(g: &mut Sudoku, count: &mut usize, itter: &mut usize) -> bool {
    *itter += 1;
    //
    // if *count > 1 {
    //     return false
    // }

    if let Some(next_idx) = g.data.iter().position(|x| x.is_none())  {
        for num in 1..=9 {
            if g.is_valid_move(next_idx, num) {
                println!("idx: {}, num: {}", next_idx, num);
                g.set_cell(next_idx, Some(num));

                if is_unique(g, count, itter) && *count > 1 {
                    println!("after is_unique {}", count);
                    return false;
                }

                g.set_cell(next_idx, None);
            }
        }
    } else {
        *count += 1;
    }

    return *count <= 1;
}

fn main() {
    let file = fs::read_to_string(PathBuf::from("output.txt")).unwrap();
    let mut games: Vec<Sudoku> = file.lines().map(|line| Sudoku::from(line)).collect();
    let game = &mut games[0]; 

    let mut values: Vec::<(CellIdx, u8)> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..30 {
        let mut idx = rng.gen_range(0..81);
        while values.iter().any(|(x,_)| x == &idx) {
            idx = rng.gen_range(0..81);
        }
        if let Some(value) = game.get_cell(idx) {
            values.push((idx, value));
            game.set_cell(idx, None);
        }
    }

    game.print();

    let mut solution_count = 0; 
    let mut itters = 0;
    let mut g = game.clone();

    let is_unique = is_unique(&mut g, &mut solution_count, &mut itters);
    println!("is unique {}", is_unique);
    println!("itters {}", itters);

    g.print();

    // while has_more_soutions(&game) {
    //     
    // }
}
