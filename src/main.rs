mod sudoku;
use sudoku::Sudoku;

fn main() {
    let mut sdk = Sudoku::new(None);
    sdk.fill_grid(0);
    println!("{}", sdk);
}
