pub const ROWS: usize = 512; // 2^9
pub const COLS: usize = ROWS;
pub const AREA: usize = ROWS * COLS;

type Cell = bool;

pub trait Grid {
  fn new() -> Self;
  fn get(&self, row: usize, col: usize) -> Cell;
  fn set(&mut self, row: usize, col: usize, val: Cell);
}

pub struct VecOfVecs([[Cell; COLS]; ROWS]);
pub struct FlatVec([Cell; AREA]);
pub struct Hilbert([Cell; AREA]);

impl Grid for VecOfVecs {
  fn new() -> Self {
    Self([[false; ROWS]; COLS])
  }
  fn get(&self, row: usize, col: usize) -> Cell {
    self.0[row][col]
  }
  fn set(&mut self, row: usize, col: usize, val: Cell) {
    self.0[row][col] = val;
  }
}

impl Grid for FlatVec {
  fn new() -> Self {
    FlatVec([false; AREA])
  }
  fn get(&self, row: usize, col: usize) -> Cell {
    self.0[row * COLS + col]
  }
  fn set(&mut self, row: usize, col: usize, val: Cell) {
    self.0[row * COLS + col] = val;
  }
}

impl Hilbert {
  // Standard Hilbert (x,y)->d for n×n where n is a power of two (here n=32768).
  #[inline]
  fn hilbert_xy_to_d(n: usize, mut x: usize, mut y: usize) -> usize {
    let mut d = 0usize;
    let mut s = n >> 1;
    while s > 0 {
      let rx = if (x & s) != 0 { 1usize } else { 0usize };
      let ry = if (y & s) != 0 { 1usize } else { 0usize };

      // distance increment
      d += s * s * ((3 * rx) ^ ry);

      // rotate/flip
      if ry == 0 {
        if rx == 1 {
          x = n - 1 - x;
          y = n - 1 - y;
        }
        core::mem::swap(&mut x, &mut y);
      }
      s >>= 1;
    }
    d
  }

  #[inline]
  fn idx(row: usize, col: usize) -> usize {
    // x = col, y = row
    Self::hilbert_xy_to_d(COLS, col, row)
  }
}

impl Grid for Hilbert {
  fn new() -> Self {
    Hilbert([false; AREA])
  }
  fn get(&self, row: usize, col: usize) -> Cell {
    self.0[Self::idx(row, col)]
  }
  fn set(&mut self, row: usize, col: usize, val: Cell) {
    let i = Self::idx(row, col);
    self.0[i] = val;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  fn test_all_methods(mut grid: impl Grid) {
    for i in 0..COLS {
      for j in 0..ROWS {
        let val = grid.get(i, j);
        assert_eq!(val, false, "Newly initialized variable is all zeroes");
        grid.set(i, j, true);
        let val = grid.get(i, j);
        assert_eq!(val, true, "Setter worked");
      }
    }
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