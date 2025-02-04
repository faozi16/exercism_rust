
pub fn annotate(minefield: &[&str]) -> Vec<String> {

    if minefield.is_empty() {
        return vec![];
    }

    let mut grid: Vec<Vec<char>> = minefield.iter().map(|&row| row.chars().collect()).collect();
    
    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == '*' {
                let adjacents = [
                    (row as isize - 1, col as isize - 1),
                    (row as isize - 1, col as isize),
                    (row as isize - 1, col as isize + 1),
                    (row as isize, col as isize - 1),
                    (row as isize, col as isize + 1),
                    (row as isize + 1, col as isize - 1),
                    (row as isize + 1, col as isize),
                    (row as isize + 1, col as isize + 1),
                ];

                for &(adj_row, adj_col) in &adjacents {
                    if adj_row >= 0 && adj_row < rows as isize && adj_col >= 0 && adj_col < cols as isize {
                        let adj_row = adj_row as usize;
                        let adj_col = adj_col as usize;
                        if grid[adj_row][adj_col] != '*' {
                            grid[adj_row][adj_col] = if grid[adj_row][adj_col] == ' ' {
                                '1'
                            } else {
                                char::from_digit(grid[adj_row][adj_col].to_digit(10).unwrap() + 1, 10).unwrap()
                            };
                        }
                    }
                }
            }
        }
    }
    grid.iter().map(|row| row.iter().collect::<String>()).collect()
}
