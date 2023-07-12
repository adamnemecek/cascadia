#![feature(type_alias_impl_trait)]

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
mod hom;
mod ideal;
mod incidence;
mod intfactorization;
mod iwahori_hecke;
mod linearalgebra;
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
        hom::*,
        ideal::*,
        incidence::*,
        intfactorization::*,
        iwahori_hecke::*,
        linearalgebra::*,
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
