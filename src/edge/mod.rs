/// Edges are transactions plus metadata.  They contain references to the source
/// and destination nodes, the transaction amount, and the call / subcall from
/// the blockchain.
///  
/// Edges are single-purpose, meaning that multiple edges can exist between
/// the same nodes, but they each represent a different transaction or flow.
/// For example, if Alice sends Bob 1 ETH and then later sends Bob 2 ETH
/// there would be two edges between Alice and Bob, one for each transaction.
pub struct Edge {}
