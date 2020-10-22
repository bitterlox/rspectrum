use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub block_hash: String,
    pub block_number: i64,
    pub from: String,
    pub gas: i64,
    pub gas_price: i64,
    pub hash: String,
    pub input: String,
    pub nonce: i64,
    pub r: String,
    pub s: String,
    pub to: String,
    pub transaction_index: i64,
    pub v: String,
    pub value: i64,
}

impl Transaction {
    fn is_token_transfer(&self) -> bool {
        if self.input.len() < 10 {
            return false;
        }
        match self.input.as_str().split_at(10).0 {
            "0x" | "0x0" | "0x00" => false,
            //  transfer | transferFrom | sweep | mint
            "0xa9059cbb" | "0x23b872dd" | "0x6ea056a9" | "0x40c10f19" => true,
            _ => false,
        }
    }
    // fn is_contract_call(&self) -> bool {
    //     if let Self::Parsed(tx) = self {
    //         tx.is_contract_call() && tx.contract_address == "" && tx.input != "0x"
    //     } else {
    //         false
    //     }
    // }
    // fn is_contract_deploy_txn(&self) -> bool {
    //     self.contract_address != ""
    // }
    // fn get_token_transfer(&self) -> super::ERC20TokenTransfer {
    //     super::ERC20TokenTransfer::parse_from_str(self).unwrap()
    // }
}
