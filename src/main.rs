use std::fmt;
use rand::{thread_rng, Rng};
use std::ops::Range;
use std::char;

type Cell = Option<u8>;

struct Board  {
    data: Vec<Cell>,
}

// Board [ Row [ 1,2,3,4,5,6,7,8,9], Row [ 1,2,3,4,5,6,7,8,9], ...];
//
// Row [1,2,3, 4,5,6, 7,8,9]
// Row [1,2,3, 4,5,6, 7,8,9]
// Row [1,2,3, 4,5,6, 7,8,9]
// Row [1,2,3, 4,5,6, 7,8,9]
// Row [1,2,3, 4,5,6, 7,8,9]
// Row [1,2,3, 4,5,6, 7,8,9]
// Row [1,2,3, 4,5,6, 7,8,9]
// Row [1,2,3, 4,5,6, 7,8,9]
// Row [1,2,3, 4,5,6, 7,8,9]

impl Board {
    fn new() -> Board {
        return Board { 
            data: vec![None; 81]
        }
    }

    // fn is_valid_move(board: &Board, row: usize, col: usize, num: u8) -> bool {
    //     // Check if you can add it to the row
    //     // Check if you can add it to the column
    //     for i in 0..9 {
    //         if board.data[row][i] == Some(num) || board.data[i][col] == Some(num) {
    //             return false;
    //         }
    //     }
    //
    //     // Check if you can add it to the box
    //     // row modulo 3 
    //     // if row 0 -> row 0 start
    //     // if row 1 -> row 0 start
    //     // if row 2 -> row 0 start
    //     // if row 3 -> row 3 start
    //     // if col 0 -> col 0 start
    //     // if col 1 -> col 0 start
    //     // if col 2 -> col 0 start
    //     // if col 3 -> col 1 start
    //     let row_start_idx = row - row % 3;
    //     let col_start_idx = col - col % 3;
    //
    //     for r_idx in 0..3 {
    //         for c_idx in 0..3 {
    //             if board.data[row_start_idx + r_idx][col_start_idx + c_idx] == Some(num) {
    //                 return false;
    //             }
    //         }
    //     }
    //
    //     return true;
    // }
}
//
// fn stringify(row: &Vec<Cell>) -> String {
//     return row.into_iter()
//         .enumerate()
//         .map(|(idx, val)| {
//             let space = if idx % 3 == 2 {" ".to_string()} else { "".to_string() };
//
//             let value = match val {
//                 Some(item) => item.to_string(),
//                 None => '_'.to_string(),
//             };
//
//             return format!("{}{}", value, space);
//         }).collect();
// }
//
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut value = String::new();

        for (idx, x) in self.data.iter().enumerate() {
            // let row_idx = idx / 9;
            let is_box_boundary = idx % 3 == 2;
            let is_last = idx % 9 == 8;

            let ch = match x {
                Some(item) => char::from_digit(*item as u32, 10).unwrap(), 
                None => '_',
            };
            value.push(ch);

            if is_box_boundary {
                value.push(' ');
            }
            if is_last {
                value.push('\n');
            }
        }

        return write!(f, "{}", value);
    }
}

fn print_col(grid: &Vec<u8>, idx: u8) {
    for row in 0..9 {
        let coord = row * 9 + idx;
        println!("{}", grid[coord as usize]);
    }
}

fn print_row(grid: &Vec<u8>, idx: u8) {
    for col in 0..9 {
        let coord = idx * 9 + col;
        print!("{}", grid[coord as usize]);
    }
    println!("");
}

fn print_square(grid: &Vec<u8>, idx: u8) {
    let start_row = (idx / 3) * 3;
    let start_col = (idx % 3 ) * 3;

    for row_idx in start_row..(start_row + 3) {
        for col_idx in start_col..(start_col + 3) {
            let coord = row_idx * 9 + col_idx;
            print!("{}", grid[coord as usize]);
        }
        print!("\n");
    }
}

fn print_grid(grid: &Vec<u8>) {
    let mut value = String::new();

    for (idx, x) in grid.iter().enumerate() {
        let is_box_boundary = idx % 3 == 2;
        let is_last = idx % 9 == 8;
        
        if idx % 9 == 0 {
            value.push('|');
        }

        value.push(char::from_digit(*x as u32, 10).unwrap());
        value.push('|');

        if is_box_boundary {
            value.push_str("   ");

            if !is_last {
                value.push('|');
            }
        }
        if is_last {
            value.push('\n');
        }
    }

    println!("{}", value);
}


// type Square = Option<u8>;
// type Grid = Vec<Square>;

// fn conflicts(squares: &Grid, cursor: usize, value: u8) -> bool {
//     true
// }

pub fn new_grid() -> Vec<u8> {
    let mut grid = Vec::new(); 
    // let mut available: Vec<u8> = (1..=9).collect();
    let mut cursor = 0;

    while cursor < 81 {
        let idx = thread_rng().gen_range(0..=9).try_into().ok();
        grid.push(idx);
        cursor += 1;
        // if !available.get(cursor).is_some() {
        //     let idx = thread_rng().gen_range(0..available[cursor].len());
            // let value = available[cursor][idx];
            //
            // if !conflicts(&squares, cursor, value) {
            //
            // }
        // }
    }
     // println!("grid::: {:?}", grid);

    return grid.iter().map(|x| x.unwrap()).collect();
}

fn main() {
    let board = Board::new(); 
    println!("{}", board);

    let grid = new_grid();
    print_grid(&grid);
    print_square(&grid, 1);
    print_row(&grid, 0);
    print_col(&grid, 0);
}
