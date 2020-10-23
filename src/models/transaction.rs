use crate::models::Storable;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub block_hash: String,
    pub block_number: i64,
    pub hash: String,
    pub timestamp: u64,
    pub input: String,
    pub value: String,
    pub gas: u128,
    pub gas_price: u128,
    pub nonce: String,
    pub transaction_index: i64,
    pub from: String,
    pub to: String,
    pub status: bool,
    pub gas_used: u128,
    pub contract_address: String,
    pub logs: Vec<super::raw::tx_receipt::Log>,
}

impl Storable for Transaction {
    fn collection() -> String {
        super::TXNS.to_string()
    }
}
