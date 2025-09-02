# One-day Experiment: Space-filling Curves and Data-oriented Design

## Background

I have been thinking about the Hilbert Curve and cache locality. I took advantage of my Labor Day off to experiment with these ideas a bit.

**Space-filling curves** are fractal, one-dimensional arrays mapped to two-dimensional space. The most famous space-filling curve is the [Hilbert Curve](https://youtu.be/3s7h2MHQtxc?si=DXUMUTtjUcxLxNoD), which I first encountered in an excellent [YouTube video by SuckerPinch](https://youtu.be/JcJSW7Rprio?si=b3YLp4kfd8jzxxH2&t=371).

**Data-oriented Design (DOD)** is an approach to code structure which leads to memory layouts that are friendly to the CPU cache. Importantly for this project, DOD generally favors simple arrays over more complex data structures. This is because array lookups tend to query for [spatially adjacent](https://en.wikipedia.org/wiki/Locality_of_reference) data that can fit in a CPU cache.

Based on what I've read, I suspect it's possible to optimize a grid-based, lookup-heavy process by mapping the 

## Question

Which representation of a two-dimensional array is best for performance (lookup speed)?

## Hypothesis

Mapping a two-dimensional array to a space-filling curve leads to better cache locality and thus faster lookups.

## Methods

I simulate Conway's Game of Life on a large, two-dimensional ($n × m$) grid. Each cell in a grid stores a single boolean value. The grid is laid out in-memory using one of the following strategies:

* **Array-of-arrays**
  Cells are assigned to one of $n$ arrays with $m$ cells each. Cells are accessed with `grid[row][column]`.
* **Flattened array**
  Same as array-of-arrays, except that each of the $n$ arrays is appended to the next, such that all cells are stored in a single array of length $n × m$. Cells are accessed with `grid[(row × m) + column]`.
* **Hilbert curve**
  Each cell is mapped to a unique point along a Hilbert Curve using a mapping function `hilbert(row, column)`. Cells are accessed using `grid[hilbert(row, column)]`.

I seed each Game of Life board with a preset initial condition. I measure the performance of each memory layout by testing _how long it takes to simulate 100 iterations of Conway's Game of Life._ Using the [`criterion`](https://bheisler.github.io/criterion.rs/book/) testing framework, I repeat this benchmark a statistically significant number of times.

I ran benchmarks on EC2:
- **Image:** Ubuntu (ubuntu/images/hvm-ssd-gp3/ubuntu-noble-24.04-amd64-server-20250821)
- **Instance Type:** t2.large
- **Memory:** 8Gb
- **vCPUs:** 2

## Results

|Data Layout|Median Benchmark Time|% Change|
|---|---|---|
|Array of Arrays|86.631 ms|-|
|Flat Array|118.48 ms|37% slower|
|Hilbert Curve|4.1792 **s**|4700% slower|

_Note that the Hilbert Curve times are measured in **seconds**, not milliseconds._

> [Read the full criterion report](https://stephenlf.github.io/Space-filling-Curves-and-Data-oriented-Design/cache-locality-experiment/report/)

## Discussion

The most naive memory layout--an array of arrays--is the best memory layout in terms of lookup speed. This is _not_ what I hypothesized.

One issue with the Flat Array and Hilbert Curve approaches is that every lookup requires some math operation to transform each 2D coordinate pair into a 1D vector lookup. The Flat Array's transformation function `(row × m) + column` is relatively simple, hence a relatively small slowdown. The Hilbert Curve transformation is much more involved, especially since I took very little effort to optimize it. Perhaps a [faster Hilbert Curve implementation](https://github.com/becheran/fast-hilbert) would fare better?

I also suspect that the Rust compiler is messing with things a bit. Are my memory layouts getting flattened away or otherwise shuffled?

The biggest issue with my hypothesis is that I assumed that the array-of-arrays layout has cache misses. This isn't necessarily true. The modern CPU L1 cache is 8-64kb in size, which is more than enough to store the 3 or so rows we need to find a cell's neighbors. I only used a 512×512 boolean grid, so three rows of data could theoretically be packed into 512 * 3 = 1536 bits. Depending on how tightly each element is packed, it's entirely possible that the whole grid fits in the L1 cache.