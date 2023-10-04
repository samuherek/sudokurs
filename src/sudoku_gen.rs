use rand::{thread_rng, Rng};

type Square = Option<u8>;
type Grid = Vec<Square>;

fn conflicts(squares: &Grid, cursor: usize, value: u8) -> bool {
    
}

pub fn new_grid() -> Grid {
    let mut squares: Grid = vec![None; 81];
    let mut available: Vec<Vec<u8>> = vec![vec![1,2,3,4,5,6,7,8,9]; 81];
    let mut cursor = 0;

    while cursor < 81 {
        if !available[cursor].is_empty() {
            let idx = thread_rng().gen_range(0..available[cursor].len());
            let value = available[cursor][idx];

            if !conflicts(&squares, cursor, value) {

            }
        }
    }

    return squares;
}
