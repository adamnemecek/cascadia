# commutative rings

Port of https://github.com/KlausC/CommutativeRings

https://github.com/wbhart/AbstractAlgebra.jl/blob/4ca1e71024c8010c810514d940f02774d062482b/src/NCPoly.jl#L13


Of importance is the https://en.wikipedia.org/wiki/Lagrange_inversion_theorem

Ideally, we would have this api

This crate relies on macros quite a bit for it's API.

The api is inspired by oscar.jl, singular, gap, 

```rust
poly!(QQ, gens!(x, y, z)) == 


let (P, [x,y,z]) = poly!(QQ, x, y, z);
```
<!-- 
#[macro_export]
macro_rules! matrix {
    () => {
        {
            // Handle the case when called with no arguments, i.e. matrix![]
            use $crate::matrix::Matrix;
            Matrix::new(0, 0, vec![])
        }
    };
    ($( $( $x: expr ),*);*) => {
        {
            use $crate::matrix::Matrix;
            let data_as_nested_array = [ $( [ $($x),* ] ),* ];
            let rows = data_as_nested_array.len();
            let cols = data_as_nested_array[0].len();
            let data_as_flat_array: Vec<_> = data_as_nested_array.into_iter()
                .flat_map(|row| row.into_iter())
                .cloned()
                .collect();
            Matrix::new(rows, cols, data_as_flat_array)
        }
    }
} -->
