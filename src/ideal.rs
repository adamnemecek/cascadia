use crate::prelude::*;

pub struct Ideal<R: Ring> {
    base: Vec<R>,
}

impl<R: Ring> Ideal<R> {
    pub fn new(base: Vec<R>) -> Self {
        Self { base }
    }
}
