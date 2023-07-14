#![feature(type_alias_impl_trait)]

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
mod group;
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
mod monomial;
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
        group::*,
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
        monomial::*,
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
