# cascadia

## implement


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