#![feature(type_alias_impl_trait)]

mod abstract_;
mod coxeter;
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
mod hom;
mod hyperplane;
mod ideal;
mod incidence;
mod intfactorization;
mod iwahori_hecke;
mod laurent;
mod linearalgebra;
mod matrix;
mod mobius;
mod multivarpolynom;
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
mod typevars;
mod univarpolynom;
mod zz;
mod zzmod;

pub mod prelude {
    pub use crate::{
        abstract_::*,
        coxeter::*,
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
        hom::*,
        hom::*,
        hyperplane::*,
        ideal::*,
        incidence::*,
        intfactorization::*,
        iwahori_hecke::*,
        laurent::*,
        linearalgebra::*,
        matrix::*,
        mobius::*,
        multivarpolynom::*,
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
