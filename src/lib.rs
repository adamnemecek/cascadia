#![feature(slice_group_by)]
#![feature(type_alias_impl_trait)]

use ext::DivRem;

mod abstract_;
mod binomial;
mod bseries;
mod coalgebra;
mod complex;
mod conat;
mod control;
mod coxeter;
mod dendriform;
mod determinant;
mod dual;
mod enumerations;
mod expr;
mod ext;
mod factorization;
mod feedback;
mod fraction;
mod galoisfields;
mod gamma;
mod generic;
mod graph;
mod groebner;
mod group;
mod hankel;
mod hodge;
mod hom;
mod hyperplane;
mod hyperplane_arrangements;
mod ideal;
mod incidence;
mod interval;
mod intfactorization;
mod iwahori_hecke;
mod kronecker;
mod laurent;
mod linearalgebra;
mod matrix;
mod matroid;
mod mobius;
mod mobius_transform;
mod monomial;
mod multivarpolynom;
mod ncpoly;
mod nominial_set;
mod numbertheoretical;
mod ordered_counter;
mod poset;
mod powerseries;
mod promoteconvert;
mod qq;
mod quotient;
mod rationalcanonical;
mod resultant;
mod ring;
mod ringtypes;
mod shift;
mod shuffle;
mod sign;
mod sign_vector;
mod smash_product;
mod specialseries;
mod species;
mod stream;
mod symbol;
mod symmetric_matrix;
mod taylor;
mod term;
mod toeplitz_matrix;
mod tutte;
mod typevars;
mod umbral;
mod univarpolynom;
mod young_tableau;
mod zonotope;
mod zz;
mod zzmod;

pub mod prelude {
    pub use crate::{
        abstract_::*,
        binomial::*,
        bseries::*,
        coalgebra::*,
        complex::*,
        conat::*,
        control::*,
        coxeter::*,
        dendriform::*,
        determinant::*,
        dual::*,
        enumerations::*,
        expr::*,
        ext::*,
        factorization::*,
        feedback::*,
        fraction::*,
        galoisfields::*,
        generic::*,
        graph::*,
        groebner::*,
        group::*,
        hankel::*,
        hodge::*,
        hom::*,
        hyperplane::*,
        hyperplane_arrangements::*,
        ideal::*,
        incidence::*,
        interval::*,
        intfactorization::*,
        iwahori_hecke::*,
        kronecker::*,
        laurent::*,
        linearalgebra::*,
        matrix::*,
        matroid::*,
        mobius::*,
        mobius_transform::*,
        monomial::*,
        multivarpolynom::*,
        ncpoly::*,
        nominial_set::*,
        numbertheoretical::*,
        ordered_counter::*,
        poset::*,
        powerseries::*,
        promoteconvert::*,
        qq::*,
        quotient::*,
        rationalcanonical::*,
        resultant::*,
        ring::*,
        ringtypes::*,
        shuffle::*,
        sign::*,
        sign_vector::*,
        smash_product::*,
        specialseries::*,
        species::*,
        stream::*,
        symbol::*,
        symmetric_matrix::*,
        taylor::*,
        term::*,
        toeplitz_matrix::*,
        tutte::*,
        typevars::*,
        umbral::*,
        univarpolynom::*,
        young_tableau::*,
        zonotope::*,
        zz::*,
        zzmod::*,
    };
}

pub trait AbstractMatrix:
    std::ops::Index<(usize, usize)>
{
    //
}
