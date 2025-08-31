pub const ROWS: usize = 1000;
pub const COLS: usize = 1000;
pub const AREA: usize = ROWS * COLS;

pub trait Grid {
  fn new() -> Self;
  fn get(&self, row: usize, col: usize) -> bool;
  fn set(&mut self, row: usize, col: usize, val: bool);
}

pub struct VecOfVecs([[bool; COLS]; ROWS]);
struct FlatVec([bool; AREA]);
struct Hilbert([bool; AREA]);

impl Grid for VecOfVecs {
  fn new() -> Self {
    Self([[false; ROWS]; COLS])
  }
  fn get(&self, row: usize, col: usize) -> bool {
    self.0[row][col]
  }
  fn set(&mut self, row: usize, col: usize, val: bool) {
    self.0[row][col] = val;
  }
}

impl Grid for FlatVec {
  fn new() -> Self {
    FlatVec([false; AREA])
  }
  fn get(&self, row: usize, col: usize) -> bool {
    todo!()
  }
  
  fn set(&mut self, row: usize, col: usize, val: bool) {
    todo!()
  }
}

impl Grid for Hilbert {
  fn new() -> Self {
    Hilbert([false; AREA])
  }
  fn get(&self, row: usize, col: usize) -> bool {
    todo!()
  }
  
  fn set(&mut self, row: usize, col: usize, val: bool) {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use crate::*;

  fn test_all_methods(mut grid: impl Grid) {
    let one_one = grid.get(1, 1);
    assert_eq!(one_one, false, "Newly initialized variable is all zeroes");
    grid.set(1, 1, true);
    let one_one = grid.get(1, 1);
    assert_eq!(one_one, true, "Setter worked");
  }

  #[test]
  fn vec_of_vecs_grid() {
    test_all_methods(VecOfVecs::new())
  }

  #[test]
  fn flat_vec_grid() {
    test_all_methods(FlatVec::new())
  }

  #[test]
  fn hilbert_grid() {
    test_all_methods(Hilbert::new())
  }
}