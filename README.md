# Space-filling Curves and Data-oriented Design

**Space-filling curves** are fractal, one-dimensional arrays mapped to two-dimensional space. The most famous space-filling curve is the [Hilbert Curve](https://youtu.be/3s7h2MHQtxc?si=DXUMUTtjUcxLxNoD), which I first encountered in an excellent [YouTube video by SuckerPinch](https://youtu.be/JcJSW7Rprio?si=b3YLp4kfd8jzxxH2&t=371).

**Data-oriented Design (DOD)** is an approach to code structure which leads to memory layouts that are friendly to the CPU cache. Importantly for this project, DOD generally favors simple arrays over more complex data structures. This is because array lookups tend to query for [spatially adjacent](https://en.wikipedia.org/wiki/Locality_of_reference) data that can fit in a CPU cache.

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

I seed each Game of Life board with a preset initial condition. I measure the performance of each memory layout by testing _how long it takes to simulate 1 million iterations of Conway's Game of Life._

I ran benchmarks on EC2:
- **Image:** Ubuntu (ubuntu/images/hvm-ssd-gp3/ubuntu-noble-24.04-amd64-server-20250821)
- **Instance Type:** t2.large
- **Memory:** 8Gb
- **vCPUs:** 2

## Results

See docs/ directory for criterion report.