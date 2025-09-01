use crate::grid;

pub struct Game(grid::Grid);

/// Add left and right together, wrapping around range [0..wraparound)
fn add_with_wraparound(left: usize, right: i32, wraparound: usize) -> usize {
  ((left as i32 + (wraparound as i32 + right)) % wraparound as i32) as usize
}

impl Game {
  pub fn new(grid: grid::Grid) -> Self {
    Self(grid)
  }

  /// Simulate a single step in the game
  pub fn increment(&mut self) {
    // Loop over 
    let grid = &mut self.0;
    for row in 0..grid::ROWS {
      for col in 0..grid::COLS {
        let cell: bool = grid.get(row, col);
        let col_left = add_with_wraparound(col, -1, grid::COLS);
        let col_right = add_with_wraparound(col, 1, grid::COLS);
        let row_up = add_with_wraparound(row, -1, grid::ROWS);
        let row_down = add_with_wraparound(row, 1, grid::ROWS);
        let num_neighbors: i32 = 
          grid.get(col_left, row_up) as i32
          + grid.get(col, row_up) as i32
          + grid.get(col_right, row_up) as i32
          + grid.get(col_left, row) as i32
          + grid.get(col_right, row) as i32
          + grid.get(col_left, row_down) as i32
          + grid.get(col, row_down) as i32
          + grid.get(col_right, row_down) as i32;
        let val = num_neighbors == 3 || (num_neighbors == 2 && cell);
        grid.set(row, col, val);
      }
    }
  }
  
  pub fn add_glider(&mut self) {
    let grid = &mut self.0;
    // Basic glider
    grid.set(1, 0, true);
    grid.set(2, 1, true);
    grid.set(3, 0, true);
    grid.set(3, 1, true);
    grid.set(3, 2, true);
  }

  pub fn bitmap(&self) -> [[u8; grid::COLS]; grid::ROWS] {
    self.0.bitmap()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  
  #[test]
  fn test_add_with_wraparound() {
    assert_eq!(add_with_wraparound(1, 2, 100), 3);
    assert_eq!(add_with_wraparound(2, 2, 3), 1);
    assert_eq!(add_with_wraparound(2, 1, 3), 0);
    assert_eq!(add_with_wraparound(10, -1, 11), 9);
    assert_eq!(add_with_wraparound(0, -1, 10), 9);
  }
}