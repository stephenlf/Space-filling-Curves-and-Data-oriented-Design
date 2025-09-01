
#![allow(unused)]
fn main() {
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use cache_locality_experiment::{game, grid};

pub fn criterion_benchmark(c: &mut Criterion) {
  let mut vec_of_vecs = game::Game::new(grid::Grid::vec_of_vecs());
  vec_of_vecs.add_glider();
  c.bench_function("vec_of_vecs 1000", |b| b.iter(|| vec_of_vecs.simulate(black_box(1000))));

  let mut flat_vec = game::Game::new(grid::Grid::flat_vec());
  flat_vec.add_glider();
  c.bench_function("flat_vec 1000", |b| b.iter(|| flat_vec.simulate(black_box(1000))));

  let mut hilbert = game::Game::new(grid::Grid::hilbert());
  hilbert.add_glider();
  c.bench_function("hilbert 1000", |b| b.iter(|| hilbert.simulate(black_box(1000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
}
