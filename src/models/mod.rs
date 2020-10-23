mod raw;
pub mod token_transfer;
mod transaction;

// const BLOCKS: &str = "blocks";
const TXNS: &str = "transactions";
// const UNCLES: &str = "uncles";
// const CONTRACTS: &str = "contracts";
// const CONTRACT_CALLS: &str = "contractcalls";
const TRANSFERS: &str = "tokentransfers";
// const FORKED_BLOCKS: &str = "forkedblocks";
// const CHARTS: &str = "charts";
// const STORE: &str = "sysstores";
// const ENODES: &str = "enodes";

pub trait Storable {
    fn collection() -> String;
}
