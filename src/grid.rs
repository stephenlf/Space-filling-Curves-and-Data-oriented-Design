use std::fmt::Display;

pub const BASE: u32 = 9;
pub const ROWS: usize = 2_usize.pow(BASE); // 2^9
pub const COLS: usize = ROWS;
pub const AREA: usize = ROWS * COLS;

type Cell = bool;

pub enum Grid {
  VecOfVecs([[Cell; COLS]; ROWS]),
  FlatVec([Cell; AREA]),
  Hilbert([Cell; AREA]),
}

impl Grid {
  pub fn vec_of_vecs() -> Self {
    Self::VecOfVecs([[false; COLS]; ROWS])
  }
  pub fn flat_vec() -> Self {
    Self::FlatVec([false; AREA])
  }
  pub fn hilbert() -> Self {
    Self::Hilbert([false; AREA])
  }
  pub fn get(&self, row: usize, col: usize) -> bool {
    match self {
      Grid::VecOfVecs(cells) => {
        cells[row][col]
      }
      Grid::FlatVec(cells) => {
        cells[row * COLS + col]
      }
      Grid::Hilbert(cells) => {
        let idx = hilbert_xy_to_d(COLS, col, row);
        cells[idx]
      }
    }
  }
  pub fn set(&mut self, row: usize, col: usize, val: Cell) {
    match self {
      Grid::VecOfVecs(cells) => {
        cells[row][col] = val;
      }
      Grid::FlatVec(cells) => {
        cells[row * COLS + col] = val;
      }
      Grid::Hilbert(cells) => {
        let idx = hilbert_xy_to_d(COLS, col, row);
        cells[idx] = val;
      }
    }
  }
  
  pub fn bitmap(&self) -> [[u8; COLS]; ROWS] {
    let mut bitmap = [[0_u8; COLS]; ROWS];
    for row in 0..ROWS {
      for col in 0..COLS {
        if self.get(row, col) {
          bitmap[row][col] = 1;
        }
      }
    }
    bitmap
  }

  pub fn load_bitmap(&mut self, bitmap: &[&[u8]]) {
    for row in 0..ROWS {
      for col in 0..COLS {
        if let Some(cell) = bitmap.get(row).map(|row| row.get(col)).flatten() {
          self.set(row, col, *cell != 0);
        } else {
          self.set(row, col, false);
        }
      }
    }
  }
}

impl Display for Grid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut strs: Vec<String> = Vec::with_capacity(ROWS);
    for row in 0..ROWS {
      let mut buf: Vec<char> = vec![' '; COLS * 2];
      for col in 0..COLS {
        if self.get(row, col) {
          buf[col*2] = '█';
          buf[col*2 + 1] = '█';
        }
      }
      strs.push(buf.into_iter().collect());
    }
    f.write_str(&strs.join("\n"))
  }
}

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

#[cfg(test)]
mod tests {
  use super::*;
  fn test_get_set(mut grid: Grid) {
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

  fn test_display(grid: &Grid) {
    let str = grid.to_string();
    let cell_chars = ROWS * COLS * 2;
    let newlines = ROWS - 1;
    assert_eq!(str.len(), cell_chars + newlines, "Two chars per cell, plus newline between each row");
  }

  fn test_bitmap(grid: &Grid) {
    let bitmap = grid.bitmap();
    assert_eq!(bitmap.len(), ROWS, "All rows allocated");
    assert_eq!(bitmap.len(), COLS, "All cols allocated");
    for row in 0..ROWS {
      for col in 0..COLS {
        assert_eq!(bitmap[row][col], 0, "Bitmap value set");
      }
    }
  }

  fn test_load_bitmap(grid: &mut Grid) {
    let bitmap = &[
      [1_u8, 0, 0],
      [0, 1, 0],
      [0, 0, 1]
    ];
    let to_load = bitmap.each_ref().map(|row| row.as_slice());
    grid.load_bitmap(&to_load);
    for row in 0..3 {
      for col in 0..3 {
        assert_eq!(bitmap[row][col] != 0, grid.get(row, col));
      }
    }
  }

  #[test]
  fn vec_of_vecs_grid() {
    test_get_set(Grid::vec_of_vecs())
  }

  #[test]
  fn flat_vec_grid() {
    test_get_set(Grid::flat_vec())
  }

  #[test]
  fn hilbert_grid() {
    test_get_set(Grid::hilbert())
  }

  #[test]
  fn vec_of_vecs_display() {
    test_display(&Grid::vec_of_vecs());
  }

  #[test]
  fn flat_vec_display() {
    test_display(&Grid::flat_vec());
  }

  #[test]
  fn hilbert_display() {
    test_display(&Grid::hilbert());
  }

  #[test]
  fn vec_of_vecs_bitmap() {
    test_bitmap(&Grid::vec_of_vecs());
  }

  #[test]
  fn flat_vec_bitmap() {
    test_bitmap(&Grid::flat_vec());
  }

  #[test]
  fn hilbert_bitmap() {
    test_bitmap(&Grid::hilbert());
  }

  #[test]
  fn vec_of_vecs_load_bitmap() {
    test_load_bitmap(&mut Grid::vec_of_vecs());
  }

  #[test]
  fn flat_vec_load_bitmap() {
    test_load_bitmap(&mut Grid::flat_vec());
  }

  #[test]
  fn hilbert_load_bitmap() {
    test_load_bitmap(&mut Grid::hilbert());
  }
}