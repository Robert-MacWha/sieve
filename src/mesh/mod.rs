use crate::edge::Edge;

/// Meshes are purpose-built filters which can each handle certain kinds of fund
/// flows.  It works by filtering through a set of edges, finding those that match
/// its criteria, adding metadata, and returning the subset.
///
/// For example, a mesh might handle all direct eth transfers, while another might handle
/// ERC-20 token transfers.
pub trait Mesh {
    fn filter(&self, edges: &Vec<Edge>) -> Vec<Edge>;
}
