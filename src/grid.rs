use std::fmt::Display;

pub const BASE: u32 = 9;
pub const ROWS: usize = 2_usize.pow(BASE); // 2^9
pub const COLS: usize = ROWS;
pub const AREA: usize = ROWS * COLS;

type Cell = bool;

pub trait Grid {
  fn new() -> Self;
  fn get(&self, row: usize, col: usize) -> Cell;
  fn set(&mut self, row: usize, col: usize, val: Cell);
}

#[derive(Debug)]
pub struct VecOfVecs([[Cell; COLS]; ROWS]);
#[derive(Debug)]
pub struct FlatVec([Cell; AREA]);
#[derive(Debug)]
pub struct Hilbert([Cell; AREA]);

fn to_string(grid: &impl Grid) -> String {
  let mut strs: Vec<String> = Vec::with_capacity(ROWS);
  for row in 0..ROWS {
    let mut buf: Vec<char> = vec![' '; COLS * 2];
    for col in 0..COLS {
      if grid.get(row, col) {
        buf[col*2] = '█';
        buf[col*2 + 1] = '█';
      }
    }
    strs.push(buf.into_iter().collect());
  }
  strs.join("\n")
}

impl Display for VecOfVecs {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&to_string(self))
  }
}
impl Display for FlatVec {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&to_string(self))
  }
}
impl Display for Hilbert {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&to_string(self))
  }
}

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
  // Standard Hilbert (x,y)->d for n×n where n is a power of two.
  #[inline]
  fn hilbert_xy_to_d(n: usize, mut x: usize, mut y: usize) -> usize {
    let mut d = 0usize;
    for i in 0..BASE {
      let s = 1 << i;
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

  #[test]
  fn test_display() {
    let hilbert = Hilbert::new();
    let str = hilbert.to_string();
    let cell_chars = ROWS * COLS * 2;
    let newlines = ROWS - 1;
    assert_eq!(str.len(), cell_chars + newlines, "Two chars per cell, plus newline between each row");
  }
}