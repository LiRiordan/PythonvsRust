# PythonvsRust

We will write a linear algebra library from scratch in Python and Rust in order to compare the inate speeds of these languages. In order to make this a fair comparison we will 
use the minimal numbers of imports in both cases. In particular, the Python library won't use numpy as this is written in C. For the Rust library we will split computations into batches of the same dimension and pass the compiler these dimensions in order to be able to use stack memory. This should allow a significant speed increase.

--- 

## Current structure: 
- main.rs
- modules
  - vectors.rs
  - matrices.rs
  - mod.rs
  - algorithms.rs

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
In order to be able to store as much as possible on the stack rather than the heap we use arrays but this requires passing the struct the length of the vector.

**Example**
```rust
let v: NdVector<3> = NdVector{coords: [0.2, -0.9, 1.4]}
```

An instance of NdVector<N> has the following functions: 
- len() -> usize
- dot_width(w: &NdVector<N>) -> f64
- norm() -> f64
- normalise()
- project_to(w: &NdVector<N>) -> NdVector<N>
- project_and_sub(w: &NdVector<N>)

We will add clear documentation for these functions at a later date.

### matrices.rs:

Defines the struct Matrix

```rust
pub struct Matrix<const N:usize, const M: usize> {
      pub entries: [[f64; N]; M]
}
```

Again the generic parameters serve to allow us to use arrays over vectors to exploit stack memory over heap memory.

**Example**
```rust
let m: Matrix<3, 2> = Matrix{entries: [[1.0, 4.2, 9.3], [-0.3, -0.6, 2.2]]}
```

An instance of Matrix<N, M> has the following functions:
- mul(w: &NdVector<N>) -> NdVector<M>
- transpose() -> Matrix<M, N>
- recover_column(j: usize) -> NdVector<M>
- recover_row(i: usize) -> NdVector<N>
- square() -> bool
- from_vectors(lv: Vec<NdVector<N>>) -> Matrix<N, M>
- gs() -> Matrix<N, M>
- has_zero_row() -> bool
- is_invertible() -> bool
- lin_indep_rows() -> bool

Again we will add more documentation for these at a later date.

---

## Future directions:

We will now start adding more linear algebra algorithms for these two structs. Currently we have the Gram-Schmidt algorithm for the rows of a matrix through the function .gs()

**Example**
```rust
let a: NdVector<2> = NdVector{coords: [1.0, 1.0]};
let b: NdVector<2> = NdVector{coords: [1.0, 0.0]};
let m: Matrix<2, 2> = Matrix::from_vectors(vec![a, b]);
println!("The Gram-Schmidt algorithm applied to the vectors a and b recovers: {:?}", m.gs());
```

We aim to implement QR, SVD and RREF in the near future.

After we have finished this Rust library we will start work on its Python equivalent in order to produce timing analysis of the two.





