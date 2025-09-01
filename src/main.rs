use std::fs::File;
use gif;
use cache_locality_experiment::{grid, game::Game};

fn build_encoder(image: &mut File) -> gif::Encoder<&mut File> {
  let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
  let mut encoder = gif::Encoder::new(image, grid::COLS as u16, grid::ROWS as u16, color_map).unwrap();
  encoder.set_repeat(gif::Repeat::Infinite).unwrap();
  encoder
}

fn main() {
  let mut image = File::create("render.gif").unwrap();
  let mut encoder = build_encoder(&mut image);
  let mut game = Game::new(grid::Grid::vec_of_vecs());
  game.add_glider();
  for _ in 0..6 {
    let bitmap = game.bitmap();
    let pixels = bitmap.as_flattened();
    assert_eq!(grid::COLS * grid::ROWS, pixels.len(), "Yo");
    let mut frame = gif::Frame::from_indexed_pixels(grid::COLS as u16, grid::ROWS as u16, pixels, None);
    frame.delay = 50;
    encoder.write_frame(&frame).unwrap();
    game.increment();
  }
}
