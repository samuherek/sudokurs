use rand::{thread_rng, Rng, seq::SliceRandom};
use std::ops::Range;
use std::{fs, fmt};
use std::path::Path;

const SDK_SIZE: usize = 9;
const SIZE_RANGE: Range<usize> = 0..SDK_SIZE;

type Cell = Option<u8>;
pub type CellIdx = usize;
type SqrIdx = u8;

#[derive(Clone)]
pub struct Sudoku {
    pub data: Vec<Cell>,
}

impl Sudoku {
    fn new(game: Option<Vec<Cell>>) -> Self {
        let mut data =  vec![None; SDK_SIZE * SDK_SIZE];

        if let Some(values) = game {
            for (i, item) in values.iter().enumerate() {
                data[i] = *item;
            }
        }

        Self {
            data
        }
    }

    fn fill_grid(&mut self, cursor: usize) -> bool {
        let is_some = self.data.get(cursor).is_some_and(|x| x.is_some());
        if cursor >= SDK_SIZE * SDK_SIZE || is_some {
            return true;
        }

        let mut seed: Vec<u8> = (1..=9).collect();
        seed.shuffle(&mut thread_rng());

        for num in seed {
            if self.is_valid_move(cursor, num) {
                self.data[cursor] = Some(num);

                if self.fill_grid(cursor + 1) {
                    return true;
                } else {
                    self.data[cursor] = None;
                }
            } 
        }

        return false
    }

    fn is_valid_move(&self, idx: CellIdx, num: u8) -> bool {
        let (row_idx, col_idx) =  idx_to_coords(idx);

        let row_start = row_idx * SDK_SIZE;
        if self.data[row_start..row_start + 9].contains(&num.into()) {
            return false;
        }

        for row in SIZE_RANGE {
            let coord = row * SDK_SIZE + col_idx;
            if self.data[coord] == Some(num) {
                return false;
            }
        }

        let sqr_idx = idx_to_sqr_idx(idx) as usize;
        let row_start = (sqr_idx) / 3 * 3;
        let col_start = (sqr_idx) % 3 * 3;

        for row_idx in row_start..(row_start + 3) {
            for col_idx in col_start..(col_start + 3) {
                let coord = row_idx * 9 + col_idx;
                if self.data[coord] == Some(num) {
                    return false;
                }
            }
        }

        true
    }

    fn set_cell(&mut self, idx: CellIdx, val: Cell) {
        self.data[idx] = val;
    }

    fn get_cell(&self, idx: CellIdx) -> Cell {
        self.data[idx] 
    }

    fn is_unique(&mut self, count: &mut usize) -> bool {
        if let Some(next_idx) = self.data.iter().position(Option::is_none)  {
            for num in 1..=9 {
                if self.is_valid_move(next_idx, num) {
                    self.set_cell(next_idx, Some(num));
                    if !self.is_unique(count) {
                        return false;
                    }
                    self.set_cell(next_idx, None);
                }
            }
        } else {
            *count += 1;
        }

        return *count <= 1;
    }

    fn prepare(&mut self) {
        let count = thread_rng().gen_range(20..50);
        for _ in 0..count {
            let idx = thread_rng().gen_range(0..81);
            self.data[idx] = None;
        }
    }

    pub fn get_grid(&self) -> Vec<Vec<Option<u8>>> {
        self.data.chunks(9).map(|x| x.to_vec()).collect()
    }

    fn print(&self) {
        let mut value = String::new();

        for (idx, x) in self.data.iter().enumerate() {
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
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: String = self.data.iter().map(get_cell_char).collect();
        write!(f, "{}", value)
    }
}

impl From<&str> for Sudoku {
    fn from(value: &str) -> Self {
        let input: Vec<Option<u8>> = value.trim().chars()
            .map(|ch| match ch {
                '1'..='9' => {
                    let num = (ch as u8) - 48;
                    Some(num)
                },
                _ => None
            })
        .collect();

        Sudoku::new(Some(input))
    }
}

fn get_cell_char(val: &Cell) -> char {
    match val {
        Some(v) => (v + 48) as char,
        None => '_',
    }
}

fn idx_to_coords(idx: usize) -> (usize, usize) {
    let row_idx = (idx as usize) / 9;
    let col_idx = (idx as usize) % 9;
    (row_idx, col_idx)
}

fn idx_to_sqr_idx(idx: usize) -> SqrIdx {
    let (row_idx, col_idx) = idx_to_coords(idx);
    if row_idx < 3 {
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
    }
}


pub fn generate(out_path: &Path, limit: usize) {
    let mut output: Vec<(Sudoku, Sudoku)> = Vec::new();

    while output.len() < limit {
        let mut sdk = Sudoku::new(None);
        sdk.fill_grid(0);
        let mut count = 0;
        let mut game = sdk.clone();
        game.prepare();

        if game.is_unique(&mut count) {
            output.push((sdk, game));
        }
    }

    let value: String = output.iter().map(|(base, play)| {
        format!("{}\n{}\n=\n", base.to_string(), play.to_string())

    }).collect();

    fs::write(out_path, value).unwrap();
}
