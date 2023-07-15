#![feature(type_alias_impl_trait)]

use ext::DivRem;

mod abstract_;
mod bseries;
mod coalgebra;
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
mod laurent;
mod linearalgebra;
mod matrix;
mod mobius;
mod monomial;
mod ordered_counter;
mod multivarpolynom;
mod ncpoly;
mod numbertheoretical;
mod powerseries;
mod promoteconvert;
mod qq;
mod quotient;
mod rationalcanonical;
mod resultant;
mod ring;
mod ringtypes;
mod shift;
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

pub mod prelude {
    pub use crate::{
        abstract_::*,
        bseries::*,
        coalgebra::*,
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
        hom::*,
        hom::*,
        hyperplane::*,
        hyperplane_arrangements::*,
        ideal::*,
        incidence::*,
        intfactorization::*,
        iwahori_hecke::*,
        laurent::*,
        linearalgebra::*,
        matrix::*,
        mobius::*,
        monomial::*,
        ordered_counter::*,
        multivarpolynom::*,
        ncpoly::*,
        numbertheoretical::*,
        powerseries::*,
        promoteconvert::*,
        qq::*,
        quotient::*,
        rationalcanonical::*,
        resultant::*,
        ring::*,
        ringtypes::*,
        specialseries::*,
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
