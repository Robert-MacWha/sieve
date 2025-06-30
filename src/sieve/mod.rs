use alloy::primitives::Address;

use crate::{edge::Edge, mesh::Mesh};

/// Sieve is the core struct which filters through transactions and finds the fund flow graph.
pub struct Sieve {
    meshes: Vec<Box<dyn Mesh>>,
}

impl Sieve {
    pub fn new(meshes: Vec<Box<dyn Mesh>>) -> Self {
        Sieve { meshes }
    }

    // Traces the fund flow graph starting from a given address.
    pub fn trace(&self, start: &Address) -> Vec<Edge> {
        todo!()
    }
}
