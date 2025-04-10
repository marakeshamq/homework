fn get_symbol(col: usize, row: usize, max_col: usize, max_row: usize) -> char {
    let is_border = row == 0 || row == max_row - 1 || col == 0 || col == max_col - 1;
    let is_diagonal = col == row || col == max_col - 1 - row;

    if is_border || is_diagonal {
        '*'
    } else {
        ' '
    }
}

fn main() {
    const COLS: usize = 50;
    const ROWS: usize = 50;

    let mut result = String::new();

    for row in 0..ROWS {
        for col in 0..COLS {
            result.push(get_symbol(col, row, COLS, ROWS));
        }
        result.push('\n');
    }

    println!("{}", result);
}
