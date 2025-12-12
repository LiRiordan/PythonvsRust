# PythonvsRust

A linear algebra library in Python and Rust written from scratch in order to compare the inate speeds of these languages. In order to make this a fair comparison these
use the minimal numbers of imports in both cases. In particular, the Python library won't use numpy as this is written in C. For the Rust library computations are into batches of the same dimension and pass the compiler these dimensions in order to be able to use stack memory. All CPython variables live in the heap so this should give a significant speed increase for Rust. 

--- 

## Current structure: 

- src
  - main.rs
  - modules
    - vectors.rs
    - matrices.rs
    - mod.rs

Along with the Rust files there are Cargo files which are used in any Rust crate. 

### main.rs:
Currently contains some example functions from the library. Have a look for some basic library uses in Rust. 

### vectors.rs:
Defines the struct NdVector 

```rust
pub struct NdVector<const N: usize> {
      pub coords: [f64; N]
}
```
In order to be able to store as much as possible on the stack rather than the heap use arrays but this requires passing the struct the length of the vector.

**Example**
```rust
let v: NdVector<3> = NdVector{coords: [0.2, -0.9, 1.4]}
```

### matrices.rs:

Defines the struct Matrix

```rust
pub struct Matrix<const N:usize, const M: usize> {
      pub entries: [[f64; N]; M]
}
```

Again the generic parameters serve to use arrays over vectors to exploit stack memory over heap memory.

**Example**
```rust
let m: Matrix<3, 2> = Matrix{entries: [[1.0, 4.2, 9.3], [-0.3, -0.6, 2.2]]}
```

---

## Future directions:

Adding more linear algebra algorithms for these two structs. Currently Gram-Schmidt algorithm is implemented for the rows of a matrix through the function .gs()

**Example**
```rust
let a: NdVector<2> = NdVector{coords: [1.0, 1.0]};
let b: NdVector<2> = NdVector{coords: [1.0, 0.0]};
let m: Matrix<2, 2> = Matrix::from_vectors(vec![a, b]);
println!("Gram-Schmidt applied to a and b: {:?}", m.gs());
```

Will implement QR, SVD and RREF in the near future.

Python library will be added after in order to produce timing analysis of the two.





