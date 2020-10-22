use bytes::Buf;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ERC20TokenTransfer {
    pub block_hash: String,
    pub block_number: i64,
    pub hash: String,
    pub timestamp: u64,
    pub from: String,
    pub to: String,
    pub value: u128,
    pub contract: String,
    pub method: String,
    pub status: bool,
    // If the token can't be recognized we give it "unknown" method and attach the input data
    pub data: String,
}

impl ERC20TokenTransfer {
    pub fn parse_from_str(tx: &super::Tx) -> Result<Self, hex::FromHexError> {
        let data = tx.input.clone();
        let (method, rest) = data.split_at(10);

        let mut params: Vec<&str> = vec![];

        //TODO: refactor this
        if rest.len() == 128 {
            let (p1, p2) = rest.split_at(64);
            params.push(p1);
            params.push(p2);
        } else if rest.len() == 192 {
            let (p1, p2, p3) = {
                let (p1, p2p3) = rest.split_at(64);
                let (p2, p3) = p2p3.split_at(64);
                (p1, p2, p3)
            };
            params.push(p1);
            params.push(p2);
            params.push(p3);
        }

        let address_from_params = move |str: &str| "0x".to_owned() + str.split_at(24).0;

        match method {
            "0xa9059cbb" => {
                let val = hex::decode(params[1])?;
                let mut tt: ERC20TokenTransfer = Default::default();

                tt.from = tx.from.clone();
                tt.to = address_from_params(params[0]);
                tt.value = bytes::Bytes::from(val).get_u128();
                tt.contract = tx.to.clone();
                tt.method = "transfer".to_string();

                Ok(tt)
            }
            "0x23b872dd" => {
                let val = hex::decode(params[2])?;
                let mut tt: ERC20TokenTransfer = Default::default();

                tt.from = address_from_params(params[0]);
                tt.to = address_from_params(params[1]);
                tt.value = bytes::Bytes::from(val).get_u128();
                tt.contract = tx.to.clone();
                tt.method = "transferFrom".to_string();

                Ok(tt)
            }
            "0x6ea056a9" => {
                let val = hex::decode(params[1])?;
                let mut tt: ERC20TokenTransfer = Default::default();

                tt.from = tx.to.clone();
                tt.to = tx.from.clone();
                tt.value = bytes::Bytes::from(val).get_u128();
                tt.contract = address_from_params(params[0]);
                tt.method = "sweep".to_string();

                Ok(tt)
            }
            "0x40c10f19" => {
                let val = hex::decode(params[1])?;
                let mut tt: ERC20TokenTransfer = Default::default();

                tt.from = "0x0000000000000000000000000000000000000000".to_string();
                tt.to = address_from_params(params[0]);
                tt.value = bytes::Bytes::from(val).get_u128();
                tt.contract = tx.to.clone();
                tt.method = "mint".to_string();

                Ok(tt)
            }
            _ => {
                let mut tt: ERC20TokenTransfer = Default::default();

                tt.method = "unknown".to_string();
                tt.data = String::from(data);

                Ok(tt)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::models::{ERC20TokenTransfer, Tx};

    #[test]
    fn test_parse_token_transfer() {
        //TODO: missing some fields, should differentiate between object we get from the daemon and objects we store
        let str = r#"
        {
          "blockHash": "0xf4035322b288b73af0cc36b5dc34162788952320bd01cb22443698537acd3951",
          "blockNumber": 1347892,
          "from": "0xb3c4e9ca7c12a6277deb9eef2dece65953d1c864",
          "gas": 150000,
          "gasPrice": 20000000000,
          "hash": "0x418d627d9cf9b85ab8fb72c8cc12a8c272e4b92280206370478cb8007f51ab33",
          "input": "0xa9059cbb000000000000000000000000b033afbf4626b107f5ca840b3a6ddc08b1abc0ea00000000000000000000000000000000000000000000000000000027172bb3a1",
          "nonce": 147562,
          "r": "0x8b89a1c3e8ce79126b5ad2b5496188d5ddad25b9048cb8c1108545f2666a1432",
          "s": "0x2e783e64db15ad11c99aa0be6aa585bad4b685e98394890ddfb705e96826d304",
          "to": "0x500684ce0d4f04abedff3e54fcf8acc5e6cfc4bd",
          "transactionIndex": 1,
          "v": "0x33",
          "value": 0
        }
        "#;

        let token_tx = r#"
         {
        	"blockNumber": NumberLong(1347892),
        	"hash": "0x418d627d9cf9b85ab8fb72c8cc12a8c272e4b92280206370478cb8007f51ab33",
        	"timestamp": NumberLong(1602942833),
        	"from": "0xb3c4e9ca7c12a6277deb9eef2dece65953d1c864",
        	"to": "0xb033afbf4626b107f5ca840b3a6ddc08b1abc0ea",
        	"value": "167892464545",
        	"contract": "0x500684ce0d4f04abedff3e54fcf8acc5e6cfc4bd",
        	"method": "transfer",
        	"status": true
        }
        "#;

        let tx: Tx = serde_json::from_str(str).unwrap();
        let token_tx = serde_json::from_str::<ERC20TokenTransfer>(token_tx);

        match token_tx {
            Ok(token_tx) => {
                let parsed_transfer: ERC20TokenTransfer =
                    ERC20TokenTransfer::parse_from_str(&tx).unwrap();

                assert_eq!(token_tx, parsed_transfer)
            }
            Err(err) => println!("error: {}", err),
        }
    }
}
