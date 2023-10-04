use std::fmt;

type Cell = Option<u8>;
type Row = Vec<Cell>;

struct Board  {
    data: Vec<Row>,
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
            data: vec![vec![None; 9]; 9]
        }
    }

    fn is_valid_move(board: &Board, row: usize, col: usize, num: u8) -> bool {
        // Check if you can add it to the row
        // Check if you can add it to the column
        for i in 0..9 {
            if board.data[row][i] == Some(num) || board.data[i][col] == Some(num) {
                return false;
            }
        }

        // Check if you can add it to the box
        // row modulo 3 
        // if row 0 -> row 0 start
        // if row 1 -> row 0 start
        // if row 2 -> row 0 start
        // if row 3 -> row 3 start
        // if col 0 -> col 0 start
        // if col 1 -> col 0 start
        // if col 2 -> col 0 start
        // if col 3 -> col 1 start
        let row_start_idx = row - row % 3;
        let col_start_idx = col - col % 3;

        for r_idx in 0..3 {
            for c_idx in 0..3 {
                if board.data[row_start_idx + r_idx][col_start_idx + c_idx] == Some(num) {
                    return false;
                }
            }
        }

        return true;
    }
}

fn stringify(row: &Row) -> String {
    return row.into_iter()
        .enumerate()
        .map(|(idx, val)| {
            let space = if idx % 3 == 2 {" ".to_string()} else { "".to_string() };

            let value = match val {
                Some(item) => item.to_string(),
                None => '_'.to_string(),
            };

            return format!("{}{}", value, space);
        }).collect();
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        //
        
        let strings: String = self.data.clone().into_iter()
            .map(|row| format!("{}\n", stringify(&row))).collect();

        println!("{}", strings);

        return write!(f, "{}", "yes");
    }
}

fn main() {
    let board = Board::new(); 
    println!("{}", board);
}
