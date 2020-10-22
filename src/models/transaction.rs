use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub block_hash: String,
    pub block_number: i64,
    pub hash: String,
    pub timestamp: i64,
    pub input: String,
    pub value: String,
    pub gas: i64,
    pub gas_price: i64,
    pub nonce: String,
    pub transaction_index: i64,
    pub from: String,
    pub to: String,
    pub status: bool,
    pub gas_used: u128,
    pub contract_address: String,
    pub logs: Vec<super::raw::tx_receipt::Log>,
}
