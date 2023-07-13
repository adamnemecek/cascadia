// pub struct Map<Domain, Codomain> {

// }

pub trait Map<Domain, Codomain> {
    fn compose<X>(&self, f: impl Map<X, Domain>);
}

// pub struct Composition<A: Map<

// pub type End<Domain> = Map<Domain, Domain>;
