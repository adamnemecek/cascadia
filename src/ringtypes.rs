pub trait RingClass {
    //
}

pub trait Ring<R: RingClass> {
    //
}

// abstract type RingClass {}
pub trait FractionRingClass: RingClass {
    //
}
trait QuotientRingClass: RingClass {
    //
}
// trait PolyRingClass <: RingClass {}
// trait Ring{T<:RingClass} {}
// trait FractionRing{S<:RingInt,T<:FractionRingClass} <: Ring{T} {}
// trait QuotientRing{S<:RingInt,T<:QuotientRingClass} <: Ring{T} {}
// trait Polynomial{S<:Ring,T<:PolyRingClass} <: Ring{T} {}
// trait RingTrait {}
// trait CommutativeRingTrait <: RingTrait {}
// trait IntegralDomainTrait <: CommutativeRingTrait {}
// trait IntegrallyClosedDomainTrait <: IntegralDomainTrait {}
// trait GCDDomainTrait <: IntegrallyClosedDomainTrait {}
// trait UniqueFactorizationDomainTrait <: GCDDomainTrait {}
// trait PrincipalIdealDomainTrait <: UniqueFactorizationDomainTrait {}
// trait EuclidianDomainTrait <: PrincipalIdealDomainTrait {}
// trait FieldTrait <: EuclidianDomainTrait {}
// trait AlgebraicallyClosedFieldTrait <: FieldTrait {}

pub struct ZZ<const N: usize> {
    //
}
