use crate::grid;

/// Add left and right together, wrapping around range [0..wraparound)
fn add_with_wraparound(right: usize, left: isize, wraparound: usize) -> usize {
  right + ((wraparound as i32 + left) % wraparound as i32) as usize
}

/// Simulate a single step in the game
pub fn increment(grid: &mut impl grid::Grid) {
  // Loop over 
  for row in 0..grid::ROWS {
    for col in 0..grid::COLS {
      let cell: bool = grid.get(row, col);
      let right_idx = col + (grid::COLS + 1) % grid::COLS; // col + 1 with wraparound
      let left_idx = col + (grid::COLS - 1) % grid::COLS; // col - 1 with wraparound
      let top_idx = row + (grid::ROWS + 1) % grid::ROWS; // row + 1 with wraparound
      let num_neighbors: i32 = (
        grid.get()
      )
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  
  #[test]
  fn test_add_with_wraparound() {

  }
}