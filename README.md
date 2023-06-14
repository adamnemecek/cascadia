# commutative rings

Port of https://github.com/KlausC/CommutativeRings

https://github.com/wbhart/AbstractAlgebra.jl/blob/4ca1e71024c8010c810514d940f02774d062482b/src/NCPoly.jl#L13


Of importance is the https://en.wikipedia.org/wiki/Lagrange_inversion_theorem

Ideally, we would have this api

This crate relies on macros quite a bit for it's API.

The api is inspired by oscar.jl, singular, gap, 


```macaulay2
R = ZZ[x,y];
S = ZZ[a,b,c];
f = map(R,S,{x^2,x*y,y^2}) // note that macaulay flips the order of things
```

I guess one option for the api do distinguish ZZ from ZZ(5) is providing a default implementation for the ZZ

and then the user can do 
```rust
poly!(ZZ::default(), x, y, z);
// vs
poly!(ZZ(4), x, y, z);


```
I wonder if i can like have a polynomial parametrized by ZZ or one that takes a ZZ as value.

```rust
trait Ring {
    fn base_ring(&self) -> ...
    fn ideal(&self) -> ...
    fn gens(&self) -> ...
    fn coeffs(&self) -> ...
}
```

# API Usage
```rust

// poly!(QQ, gens!(x, y, z)) == 


// let (P, [x,y,z]) = poly!(QQ, x, y, z);
// the above macaulay2 code would be 

let (R, [x, y]) = poly!(ZZ, x, y);
let (S, [a, b, c]) = poly!(ZZ, a, b, c);

// note that the S has three elements, same 

// the question is what does the hom funciton take? they are not Ring::Elem per se, since 
let f = hom(S, R, [x^2, x*y, y^2]);


assert!(f[c] == x * y)

hom()



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

## groebner

For noncommutative groebner reference that one ocaml package, there's also GAP and that one mathematica package from ucsd.