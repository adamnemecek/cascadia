# cascasia

Rust crate for algebra

<!-- https://github.com/habemus-papadum/HaskellForMaths/blob/4ed09d8db4ac2e19eb0180672309885f1d589155/src/Math/Algebra/NonCommutative/NCPoly.hs#L198 -->

Features:
    * abstract algebra
    * commutative algbera
    * noncommutative algbera
    * coxeter groups
        * coxeter.jl
    * tree algebras
        * dentriform.jl
    * free algbera
        * freealgbera-rs
    * combinatorics
    * signatures
    * matroids
    * vertex operator algebra
    * tutte polynomial
        * tutte.cpp
        * tutte.rs
    * hecke algebra
        * https://github.com/sagemath/sage/blob/3230f00aeb49802f99b0a3b76e770fa9d628c4e1/src/sage/algebras/iwahori_hecke_algebra.py#L122
    
    * polynomial
        * cdm
        * 

* symdpoly in scala

https://web.archive.org/web/20170107110618/http://userpages.umbc.edu/~squire/download/TaylorFit.java

The API is heavily inspired by Oscar.jl.

Other inspirations include 
* [sympycore]()
* [bruhat](https://github.com/punkdit/bruhat)


Port of https://github.com/KlausC/CommutativeRings

https://github.com/wbhart/AbstractAlgebra.jl/blob/4ca1e71024c8010c810514d940f02774d062482b/src/NCPoly.jl#L13


Of importance is the https://en.wikipedia.org/wiki/Lagrange_inversion_theorem

Ideally, we would have this api

This crate relies on macros quite a bit for it's API.

The api is inspired by oscar.jl, singular, gap, 

https://github.com/ulthiel/CoxeterGroups.jl/tree/master/src

Dentriform algebra

```macaulay2
R = ZZ[x,y];
S = ZZ[a,b,c];
f = map(R,S,{x^2,x*y,y^2})      # note that macaulay flips the order of things
f a                             # => x^2
f (a * b)                       # => x^3 y
```

I guess one option for the api do distinguish ZZ from ZZ(5) is providing a default implementation for the ZZ

and then the user can do 
```rust
poly!(ZZ::default(), x, y, z);
// vs
poly!(ZZ(4), x, y, z);


```
```
ZZPolyRingElem
  coeffs: Ptr{Nothing} @0x00006000021a3610
  alloc: Int64 2
  length: Int64 2
  parent: ZZPolyRing
    S: Symbol x
    __attrs: #undef
```

I wonder if i can like have a polynomial parametrized by ZZ or one that takes a ZZ as value.

```rust
trait Ring {
    fn base_ring(&self) -> ...
    fn ideal(&self) -> ...
    fn gens(&self) -> ...
    fn coeffs(&self) -> ...
    fn ambient(&self) -> ...
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

// the question is what does the hom funciton take? they are not Ring::Elem per se, they might be expresions
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