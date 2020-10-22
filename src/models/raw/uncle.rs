use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uncle {
    pub difficulty: i64,
    pub extra_data: String,
    pub gas_limit: i64,
    pub gas_used: i64,
    pub hash: String,
    pub logs_bloom: String,
    pub miner: String,
    pub mix_hash: String,
    pub nonce: String,
    pub number: i64,
    pub parent_hash: String,
    pub receipts_root: String,
    pub sha3_uncles: String,
    pub size: i64,
    pub state_root: String,
    pub timestamp: i64,
    pub total_difficulty: i64,
    pub transactions_root: String,
    #[serde(skip)]
    //I don't think an uncle can contain more uncles, but gubiq returns uncles with an uncle field so idk
    pub uncles: Vec<Self>,
}
