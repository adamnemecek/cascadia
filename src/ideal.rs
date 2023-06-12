use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Preflex,
    Grevlex,
}

pub struct Ideal<R: Ring> {
    base: Vec<R>,
}

impl<R: Ring> Ideal<R> {
    pub fn new(base: Vec<R>) -> Self {
        Self { base }
    }
}
