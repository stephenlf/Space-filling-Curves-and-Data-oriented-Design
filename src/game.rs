use crate::grid;

pub struct Game {
  grid: grid::Grid,
  pub generation: u32,
}

/// Add left and right together, wrapping around range [0..wraparound)
fn add_with_wraparound(left: usize, right: i32, wraparound: usize) -> usize {
  let left = left as i32;
  let wraparound = wraparound as i32;
  ((left + (wraparound + right)) % wraparound) as usize
}

impl From<grid::Grid> for Game {
  fn from(value: grid::Grid) -> Self {
    Self {
      grid: value,
      generation: 0
    }
  }
}

impl Game {
  pub fn new(grid: grid::Grid) -> Self {
    Self::from(grid)
  }

  /// Simulate a single step in the game
  pub fn increment(&mut self) {
    self.generation += 1;

    let grid = &mut self.grid;
    let mut updates: Vec<(usize, usize, bool)> = Vec::default();
    for row in 0..grid::ROWS {
      for col in 0..grid::COLS {
        let is_live: bool = grid.get(row, col);
        let col_left = add_with_wraparound(col, -1, grid::COLS);
        let col_right = add_with_wraparound(col, 1, grid::COLS);
        let row_up = add_with_wraparound(row, -1, grid::ROWS);
        let row_down = add_with_wraparound(row, 1, grid::ROWS);
        let num_neighbors: i32 = 
          grid.get(row_up, col_left) as i32
          + grid.get(row_up, col) as i32
          + grid.get(row_up, col_right) as i32
          + grid.get(row, col_left) as i32
          + grid.get(row, col_right) as i32
          + grid.get(row_down, col_left) as i32
          + grid.get(row_down, col) as i32
          + grid.get(row_down, col_right) as i32;
        let overpopulation = num_neighbors > 3;
        let underpopulation = num_neighbors < 2;
        let reproduction = !is_live && num_neighbors == 3;
        if is_live && (overpopulation || underpopulation) {
          updates.push((row, col, false));
        } else if !is_live && reproduction {
          updates.push((row, col, true));
        }
      }
    }
    for (row, col, val) in updates {
      grid.set(row, col, val);
    }
  }

  pub fn simulate(&mut self, num_generations: u32) {
    for _ in 0..num_generations {
      self.increment();
    }
  }
  
  pub fn add_glider(&mut self) {
    let glider = [
      [0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 1, 0, 0, 0],
      [0, 0, 0, 0, 1, 0, 0],
      [0, 0, 1, 1, 1, 0, 0],
      [0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0],
    ];
    let bitmap = glider.each_ref().map(|row| row.as_slice());
    self.grid.load_bitmap(&bitmap);
  }

  pub fn bitmap(&self) -> [[u8; grid::COLS]; grid::ROWS] {
    self.grid.bitmap()
  }

  pub fn inspect_grid(&self) -> &grid::Grid {
    &self.grid
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::grid::{self};
  
  #[test]
  fn test_add_with_wraparound() {
    assert_eq!(add_with_wraparound(1, 2, 100), 3);
    assert_eq!(add_with_wraparound(2, 2, 3), 1);
    assert_eq!(add_with_wraparound(2, 1, 3), 0);
    assert_eq!(add_with_wraparound(10, -1, 11), 9);
    assert_eq!(add_with_wraparound(0, -1, 10), 9);
  }

  #[test]
  fn test_underpopulation() {
    let mut grid = grid::Grid::vec_of_vecs();
    grid.set(1, 1, true);
    let mut game = Game::new(grid);
    game.increment();
    let grid = game.inspect_grid();
    assert_eq!(false, grid.get(1, 1), "Cell should have died (underpopulation)");
  }

  #[test]
  fn test_dead_with_two_neighbors() {
    let mut grid = grid::Grid::vec_of_vecs();
    grid.set(1, 1, true);
    grid.set(1, 2, true);
    let mut game = Game::new(grid);
    game.increment();
    let grid = game.inspect_grid();
    assert_eq!(false, grid.get(2, 1), "Cell should have stayed dead");
  }

  #[test]
  fn test_live_with_two_neighbors() {
    let mut grid = grid::Grid::vec_of_vecs();
    grid.set(1, 1, true);
    grid.set(1, 2, true);
    grid.set(1, 3, true);
    let mut game = Game::new(grid);
    game.increment();
    let grid = game.inspect_grid();
    assert_eq!(true, grid.get(1, 2), "Cell should have stayed live");
  }

  #[test]
  fn test_birth() {
    let mut grid = grid::Grid::vec_of_vecs();
    grid.set(1, 1, true);
    grid.set(1, 2, true);
    grid.set(1, 3, true);
    let mut game = Game::new(grid);
    game.increment();
    let grid = game.inspect_grid();
    assert_eq!(true, grid.get(2, 2), "Cell should have been born");
  }

  #[test]
  fn test_live_with_three_neighbors() {
    let mut grid = grid::Grid::vec_of_vecs();
    grid.set(1, 1, true);
    grid.set(1, 2, true);
    grid.set(1, 3, true);
    grid.set(2, 2, true);
    let mut game = Game::new(grid);
    game.increment();
    let grid = game.inspect_grid();
    assert_eq!(true, grid.get(2, 2), "Cell should have stayed live");
  }

  #[test]
  fn test_overpopulation() {
    let mut grid = grid::Grid::vec_of_vecs();
    grid.set(1, 1, true);
    grid.set(1, 2, true);
    grid.set(1, 3, true);
    grid.set(2, 1, true);
    grid.set(2, 2, true);
    let mut game = Game::new(grid);
    game.increment();
    let grid = game.inspect_grid();
    assert_eq!(false, grid.get(2, 2), "Cell should have died (overpopulation)");
  }

  fn compare_to_bitmap<const N: usize>(game: &Game, bitmap: &[[u8; N]]) {
    for row in 0..bitmap.len() {
      for col in 0..bitmap[row].len() {
        let expect = bitmap[row][col];
        let actual = game.inspect_grid().get(row, col) as u8;
        assert_eq!(expect, actual, "At generation {} ({row}, {col}), expect {expect} but got {actual}", game.generation);
      }
    }
  }

  #[test]
  fn test_simulation() {
    let mut grid = grid::Grid::vec_of_vecs();
    let bitmap = [
      [0, 1, 0],
      [0, 0, 1],
      [0, 1, 0],
    ];
    let bitmap = bitmap.each_ref().map(|row| row.as_ref());
    grid.load_bitmap(&bitmap);
    let mut game = Game::new(grid);
    assert_eq!(game.inspect_grid().get(0, 1), true, "Row 0 col 1 set");
    game.increment();
    assert_eq!(game.inspect_grid().get(0, 1), false, "Row 0 col 1 died");
  }

  #[test]
  fn test_glider() {
    let mut game = Game::new(grid::Grid::vec_of_vecs());
    game.add_glider();
    let glider = [
      [0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 1, 0, 0, 0],
      [0, 0, 0, 0, 1, 0, 0],
      [0, 0, 1, 1, 1, 0, 0],
      [0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0],
    ];
    compare_to_bitmap(&game, &glider);

    game.increment();
    let glider = [
      [0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0],
      [0, 0, 1, 0, 1, 0, 0],
      [0, 0, 0, 1, 1, 0, 0],
      [0, 0, 0, 1, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0],
    ];
    compare_to_bitmap(&game, &glider);
  }
}