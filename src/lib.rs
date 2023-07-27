#![feature(slice_group_by)]
#![feature(type_alias_impl_trait)]

use ext::DivRem;

mod abstract_;
mod bseries;
mod coalgebra;
mod complex;
mod coxeter;
mod dendriform;
mod determinant;
mod dual;
mod enumerations;
mod expr;
mod ext;
mod factorization;
mod fraction;
mod galoisfields;
mod gamma;
mod generic;
mod graph;
mod groebner;
mod group;
mod hom;
mod hyperplane;
mod hyperplane_arrangements;
mod ideal;
mod incidence;
mod intfactorization;
mod iwahori_hecke;
mod kronecker;
mod laurent;
mod linearalgebra;
mod matrix;
mod mobius;
mod mobius_transform;
mod monomial;
mod multivarpolynom;
mod ncpoly;
mod numbertheoretical;
mod ordered_counter;
mod powerseries;
mod promoteconvert;
mod qq;
mod quotient;
mod rationalcanonical;
mod resultant;
mod ring;
mod ringtypes;
mod shift;
mod smash_product;
mod specialseries;
mod symbol;
mod taylor;
mod term;
mod toeplitz;
mod tutte;
mod typevars;
mod univarpolynom;
mod zz;
mod zzmod;

mod control;
mod hankel;
mod hodge;
mod nominial_set;
mod stream;

pub mod prelude {
    pub use crate::{
        abstract_::*,
        bseries::*,
        coalgebra::*,
        complex::*,
        control::*,
        coxeter::*,
        dendriform::*,
        determinant::*,
        dual::*,
        enumerations::*,
        expr::*,
        ext::*,
        factorization::*,
        fraction::*,
        galoisfields::*,
        generic::*,
        graph::*,
        groebner::*,
        group::*,
        hankel::*,
        hodge::*,
        hom::*,
        hom::*,
        hyperplane::*,
        hyperplane_arrangements::*,
        ideal::*,
        incidence::*,
        intfactorization::*,
        iwahori_hecke::*,
        kronecker::*,
        laurent::*,
        linearalgebra::*,
        matrix::*,
        mobius::*,
        mobius_transform::*,
        monomial::*,
        multivarpolynom::*,
        ncpoly::*,
        nominial_set::*,
        numbertheoretical::*,
        ordered_counter::*,
        powerseries::*,
        promoteconvert::*,
        qq::*,
        quotient::*,
        rationalcanonical::*,
        resultant::*,
        ring::*,
        ringtypes::*,
        smash_product::*,
        smash_product::*,
        specialseries::*,
        stream::*,
        symbol::*,
        taylor::*,
        term::*,
        toeplitz::*,
        tutte::*,
        typevars::*,
        univarpolynom::*,
        zz::*,
        zzmod::*,
    };
}

pub trait AbstractMatrix:
    std::ops::Index<(usize, usize)>
{
    //
}
