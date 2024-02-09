mod sudoku;

use sudoku::Sudoku;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut output: Vec<Sudoku> = Vec::new();

    for _ in 0..10 {
        let mut sdk = Sudoku::new(None);
        sdk.fill_grid(0);
        sdk.print();
        output.push(sdk);
    }

    let value: String = output.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n");
    fs::write(PathBuf::from("output.txt"), value).unwrap();
}
