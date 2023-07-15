# cascadia

## implement


## noncommutative bases
the fundamental question is can i come up with a good way of encoding xxy that is different from 
i guess one way of approaching this would be to like sor
we might be able to distinguish two instances of the thing by couting uniques 
and then like wrap around so that if gens are a,b,c
and the thing is aab it's really,

lehmer code?
catalan numbers?

a = 0, 3, 6,
b = 1, 4, 7
c = 2, 5, 8

// noncommutative
we can then encode it with 0, 3, 4 and then we can encode to mean 

a, a, b = 0, 3, 4
b, a, b = 1, 3, 5,
b, b, a = 1, 4, 6 
a, b, a = 0, 1, 3 // most interesting case since it's incremental

### commutative
in the commutative case, we want to be able to make an easy comparison if they have the same content
so we can sort it first so that 
a, a, b = b, a, a 

* the way to compress it is to start with the last block and then starting from the first block, xor and see which ones can be combines, you are trying to redistribute until 
alternatively split it in half, and try to do the same





what we can also do is encode the things as they are 


you can't use digits for things with more than 10 items

```rust

// given something like 
fn encode(n: usize, i: impl Iterator<Item=usize>) -> u128 {
//
}

fn decode(n: usize, a: u128) -> impl Iterator<Item=usize> {
    bit(a).map(|a| a % n)
//
// digits()
}
```


should i represent this as digits?

we might then be able to compress this as a permutation





## ringhom
I'm going with how macaulay2 implements it. 

The ringhom 


You can get the generator of a ring using index
http://www2.macaulay2.com/Macaulay2/doc/Macaulay2-1.21/share/doc/Macaulay2/Macaulay2Doc/html/__us.html

macaulay2 implements the SPACE operator which is what they use for ring mapping.
 this in turns calls rawRingMapEval which is implenented in dd which takes an expr and returns an expr (see how is expr implemented)
which calls IM2_RingMap_eval_matrix



===


Implementation details of the thing


# NC vs C
The difference is how the linear combinations combine.


# Ring hom

https://github.com/gap-system/gap/blob/851dbd96a51db582ee0322b72ff28e4cf1d8d9e4/lib/ringhom.gi#L34

* sseq has an free module implementation with iamges

I guess the the trick is that like you decompose the tree until you read the leaf at which point you 

* free module
    * has a basis/generator

* ring hom implementation

m2 has a matrix

* how about instead of expresison I have a linear combination?

Maybe I can use a bitset for the 

* free module
    * is this the same as representable in haskell?


* maybe i dont need an expression but a linear combination


* no i probably need an expresison

the trick heere is that 


# mpoly

* what will happen if i keep the terms around and just multiply it 
* look at symbolica