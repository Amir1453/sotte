A simple CPU path tracing library written in Rust. 

Features:
- Sequential and Parallel Linear BVH construction via [Karras](https://research.nvidia.com/sites/default/files/publications/karras2012hpg_paper.pdf)

Project layout:
- src/geometry: Geometrical objects and intersections. Implements spheres, quads and triangle meshes.
- src/larp: Layered Acceleration with Recursive Partitions. Implements Axis-Aligned Bounding Boxes (AABB), generic Bounding Volume Hierarchies (BVH) with Surface Area Heuristic (SAH), and generic Linear BVH.
- src/math: Implements 3x10bit -> u32 and 3x21bit -> u64 Morton encodings, Radix LSD sort, rays and vectors, and surface and volume sampling.
- src/material.rs: Implements material support.
- src/renderer.rs: Implements rendering logic.
- src/scene.rs: Implements the rendering scene.
- src/texture.rs: Implements textures.

To compile:

```
cargo build --release
```

To run the examples:

```
cargo run --release --example [3body|cat|lucky|marika|rust|sphere]
```

To run the benchmarks:

```
cargo bench --bench [larpbench|mathbench]
```
