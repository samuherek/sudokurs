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

const SDK_SIZE: usize = 9;

type CellIdx = usize;
type ColIdx = u8;
type RowIdx = u8;
type SqrIdx = u8;
type Grid = Vec<Cell>;
type GridCol = [Cell; SDK_SIZE];
type GridRow = [Cell; SDK_SIZE];
type GridSqr = [Cell; SDK_SIZE];

const SIZE_RANGE: Range<usize> = 0..SDK_SIZE;

fn get_cell(grid: &Grid, idx: usize) -> Cell {
    grid.get(idx).and_then(|x| x.clone())
}

fn get_col(grid: &Grid, idx: ColIdx) -> GridCol {
    let mut cells = [None; SDK_SIZE];
    if SIZE_RANGE.contains(&idx.into()) {
        for row_idx in 0..SDK_SIZE {
           let coord = row_idx * SDK_SIZE + idx as usize;
           cells[row_idx] = get_cell(grid, coord);
        }
    }
    cells
}

fn get_row(grid: &Grid, idx: RowIdx) -> GridRow {
    let mut cells = [None; SDK_SIZE];
    let row_start = (idx as usize) * SDK_SIZE;
    if SIZE_RANGE.contains(&idx.into()) {
        for col_idx in 0..SDK_SIZE {
            let coord = row_start + col_idx;
            cells[col_idx] = get_cell(grid, coord);
        }
    }
    cells
}

fn idx_to_coords(idx: usize) -> (usize, usize) {
    let row_idx = (idx as usize) / 9;
    let col_idx = (idx as usize) % 9;
    (row_idx, col_idx)
}

fn idx_to_sqr_idx(idx: usize) -> SqrIdx {
    let (row_idx, col_idx) = idx_to_coords(idx);
    let idx = if row_idx < 3 {
        if col_idx < 3 {
            0
        } else if col_idx < 6 {
            1
        } else  {
            2
        }
    } else if row_idx < 6 {
        if col_idx < 3 {
            3
        } else if col_idx < 6 {
            4
        } else  {
            5
        }
    } else if row_idx < 9{
        if col_idx < 3 {
            6
        } else if col_idx < 6 {
            7
        } else  {
            8
        }
    } else {
        panic!("Inpossible row index");
    };
    idx
}

fn get_square(grid: &Grid, idx: SqrIdx) -> GridSqr {
    let mut cells = [None; SDK_SIZE];
    let row_start = (idx as usize) / 3 * 3;
    let col_start = (idx as usize) % 3 * 3;

    if SIZE_RANGE.contains(&idx.into()) {
        let mut cell_idx = 0;
        for row_idx in row_start..(row_start + 3) {
            for col_idx in col_start..(col_start + 3) {
                let coord = row_idx * 9 + col_idx;
                cells[cell_idx] = get_cell(grid, coord);
                cell_idx += 1;
            }
        }
    }
    cells
}

fn get_cell_char(val: &Cell) -> char {
    match val {
        Some(v) => (v + 48) as char,
        None => '_',
    }
}

fn print_cell(val: &Cell) {
    let ch = get_cell_char(val);
    println!("{}", ch);
}

fn print_col(grid: &Grid, idx: ColIdx) {
    let mut value = String::new();
    for item in get_col(grid, idx) {
        value.push(get_cell_char(&item));
        value.push('\n');
    }
    println!("{value}");
}

fn print_row(grid: &Grid, idx: RowIdx) {
    let mut value = String::new();
    for item in get_row(grid, idx) {
        value.push(get_cell_char(&item));
    }
    println!("{value}");
}

fn print_sqr(grid: &Grid, idx: u8) {
    let mut value = String::new();
    for (i, item) in get_square(grid, idx).iter().enumerate() {
        value.push(get_cell_char(&item));
        if i % 3 == 2 {
            value.push('\n');
        }
    }
    println!("{value}");
}

fn print_grid(grid: &Grid) {
    let mut value = String::new();

    for (idx, x) in grid.iter().enumerate() {
        let is_box_boundary = idx % 3 == 2;
        let is_last = idx % 9 == 8;
        
        if idx % 9 == 0 {
            value.push('|');
        }

        value.push(get_cell_char(x));
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

fn is_valid_move(grid: &Grid, idx: CellIdx, num: u8) -> bool {
    let (row_idx, col_idx) =  idx_to_coords(idx);

    let cols = get_col(grid, col_idx as u8);
    let sqr_idx = idx_to_sqr_idx(idx);
    let square = get_square(grid, sqr_idx);
    print_grid(grid);
    println!("-----");

    if cols.contains(&Some(num)) {
        return false
    }

    if square.contains(&Some(num)) {
        return false;
    }

    true
}

pub fn new_grid() -> Grid {
    let mut grid = vec![None; SDK_SIZE * SDK_SIZE]; 
    let mut seed: Vec<Vec<u8>> = (1..=9).map(|_| (1..=9).collect()).collect();
    let mut idx_cursor = 0;
    let mut overflow = 0;

    while idx_cursor < SDK_SIZE * SDK_SIZE  {
        let (row_coord, col_coord) = idx_to_coords(idx_cursor);
        println!("{idx_cursor}, {row_coord}, {col_coord}");
        if let Some(row_seed) = seed.get_mut(row_coord) {
            let row_seed_idx = thread_rng().gen_range(0..row_seed.len());
            let next_n = row_seed.swap_remove(row_seed_idx);
            println!("row seed::: {:?}", row_seed);
            println!("next num::: {next_n}");
            if is_valid_move(&grid, idx_cursor, next_n) {
                grid[idx_cursor] = Some(next_n);
                idx_cursor += 1;
            } else {
                if row_seed.len() == 0 {
                    let prev = grid[idx_cursor - 1];
                    row_seed.push(prev.unwrap());
                    grid[idx_cursor - 1] = None;
                    idx_cursor -= 1; 
                }
                row_seed.push(next_n);
                // idx_cursor -= 1;
            }
        } else {
            eprintln!("We have run out of seed rows");
        }
        overflow += 1;

    }

    grid
}

        // let idx: usize = thread_rng().gen_range(0..=9);
        // grid.push(idx);
        // if !available.get(cursor).is_some() {
        //     let idx = thread_rng().gen_range(0..available[cursor].len());
            // let value = available[cursor][idx];
            //
            // if !conflicts(&squares, cursor, value) {
            //
            // }
        // }
fn main() {
    let board = Board::new(); 
    println!("{}", board);

    let grid = new_grid();
    print_grid(&grid);
    print_sqr(&grid, 1);
    print_row(&grid, 0);
    print_col(&grid, 0);
}
