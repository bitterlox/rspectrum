use bytes::Buf;
use serde::{Deserialize, Serialize};

use crate::models::transaction::Transaction as Tx;
use crate::models::Storable;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ERC20TokenTransfer {
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

impl Storable for ERC20TokenTransfer {
    fn collection() -> String {
        super::TRANSFERS.to_string()
    }
}

impl ERC20TokenTransfer {
    pub fn parse_from_str(tx: &Tx) -> Result<Self, std::num::ParseIntError> {
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

        let address_from_params = move |str: &str| "0x".to_owned() + str.split_at(24).1;

        let mut tt: ERC20TokenTransfer = Default::default();

        tt.block_number = tx.block_number;
        tt.hash = tx.hash.clone();
        tt.timestamp = tx.timestamp;
        tt.status = tx.status;

        match method {
            "0xa9059cbb" => {
                // transfer
                tt.from = tx.from.clone();
                tt.to = address_from_params(params[0]);
                tt.value = u128::from_str_radix(params[1], 16)?;
                tt.contract = tx.to.clone();
                tt.method = "transfer".to_string();

                Ok(tt)
            }
            "0x23b872dd" => {
                // transferFrom
                tt.from = address_from_params(params[0]);
                tt.to = address_from_params(params[1]);
                tt.value = u128::from_str_radix(params[2], 16)?;
                tt.contract = tx.to.clone();
                tt.method = "transferFrom".to_string();

                Ok(tt)
            }
            "0x6ea056a9" => {
                // sweep
                tt.from = tx.to.clone();
                tt.to = tx.from.clone();
                tt.value = u128::from_str_radix(params[1], 16)?;
                tt.contract = address_from_params(params[0]);
                tt.method = "sweep".to_string();

                Ok(tt)
            }
            "0x40c10f19" => {
                // mint
                tt.from = "0x0000000000000000000000000000000000000000".to_string();
                tt.to = address_from_params(params[0]);
                tt.value = u128::from_str_radix(params[1], 16)?;
                tt.contract = tx.to.clone();
                tt.method = "mint".to_string();

                Ok(tt)
            }
            _ => {
                tt.method = "unknown".to_string();
                tt.data = data;

                Ok(tt)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::models::token_transfer::ERC20TokenTransfer;
    use crate::models::transaction::Transaction;

    fn test_parse_transfers(canon_tx: &str, canon_tt: &str) {
        let tx: Transaction = serde_json::from_str(canon_tx).unwrap();
        let token_tx = serde_json::from_str::<ERC20TokenTransfer>(canon_tt);

        match token_tx {
            Ok(tt) => {
                let parsed_transfer: ERC20TokenTransfer =
                    ERC20TokenTransfer::parse_from_str(&tx).unwrap();

                assert_eq!(tt, parsed_transfer);
            }
            Err(err) => panic!(err.to_string()),
        }
    }

    #[test]
    fn test_parse_transfer() {
        //TODO: missing some fields, should differentiate between object we get from the daemon and objects we store
        let tx = r#"
        {
          "blockHash": "0xf4035322b288b73af0cc36b5dc34162788952320bd01cb22443698537acd3951",
          "blockNumber": 1347892,
          "hash": "0x418d627d9cf9b85ab8fb72c8cc12a8c272e4b92280206370478cb8007f51ab33",
          "timestamp": 1602942833,
          "input": "0xa9059cbb000000000000000000000000b033afbf4626b107f5ca840b3a6ddc08b1abc0ea00000000000000000000000000000000000000000000000000000027172bb3a1",
          "value": "0",
          "gas": 150000,
          "gasPrice": 20000000000,
          "nonce": "0x2406a",
          "transactionIndex": 1,
          "from": "0xb3c4e9ca7c12a6277deb9eef2dece65953d1c864",
          "to": "0x500684ce0d4f04abedff3e54fcf8acc5e6cfc4bd",
          "status": true,
          "gasUsed": 37201,
          "contractAddress": "",
          "logs": []
        }
        "#;

        let tt = r#"
         {
        	"blockNumber": 1347892,
        	"hash": "0x418d627d9cf9b85ab8fb72c8cc12a8c272e4b92280206370478cb8007f51ab33",
        	"timestamp": 1602942833,
        	"from": "0xb3c4e9ca7c12a6277deb9eef2dece65953d1c864",
        	"to": "0xb033afbf4626b107f5ca840b3a6ddc08b1abc0ea",
        	"value": 167892464545,
        	"contract": "0x500684ce0d4f04abedff3e54fcf8acc5e6cfc4bd",
        	"method": "transfer",
        	"status": true,
        	"data": ""
        }
        "#;

        test_parse_transfers(tx, tt);
    }

    #[test]
    fn test_parse_transferFrom() {
        let tx = r#"
        {
           "blockHash": "0xfc9d19beccfb08331ef15250c3d479289ecf193f7b13e79ae9da64624f316462",
           "blockNumber": 45569,
           "hash": "0xb72d6fd9090b87957f39ee89302599cc77d7c9f74958c3fec2856278d22e465f",
           "timestamp": 1489589946,
           "input": "0x23b872dd000000000000000000000000f25e811ac2258f9890cad60318799875831346b7000000000000000000000000b3c4e9ca7c12a6277deb9eef2dece65953d1c8640000000000000000000000000000000000000000000000000000000001c9c383",
           "value": "0",
           "gas": 178907,
           "gasPrice": 20000000000,
           "nonce": "0x1fe",
           "transactionIndex": 0,
           "from": "0xb3c4e9ca7c12a6277deb9eef2dece65953d1c864",
           "to": "0x4b4899a10f3e507db207b0ee2426029efa168a67",
           "status": false,
           "gasUsed": 28907,
           "contractAddress": "",
           "logs": []
        }
        "#;

        let tt = r#"
         {
           "blockNumber": 45569,
           "hash": "0xb72d6fd9090b87957f39ee89302599cc77d7c9f74958c3fec2856278d22e465f",
           "timestamp": 1489589946,
           "from": "0xf25e811ac2258f9890cad60318799875831346b7",
           "to": "0xb3c4e9ca7c12a6277deb9eef2dece65953d1c864",
           "value": 30000003,
           "contract": "0x4b4899a10f3e507db207b0ee2426029efa168a67",
           "method": "transferFrom",
           "status": false,
           "data": ""
        }
        "#;

        test_parse_transfers(tx, tt);
    }
    #[test]
    fn test_parse_sweep() {
        let tx = r#"
        {
          "blockHash": "0x37c636be5ffec160da2f6838f6eb4a667aee92874c359d0b930bea842d60ac26",
          "blockNumber": 1353398,
          "hash": "0x22c8f97e6e3a496fa3c090f793cc36629c55f1817356b827e6c3a77cb9b609e4",
          "timestamp": 1603429238,
          "input": "0x6ea056a9000000000000000000000000500684ce0d4f04abedff3e54fcf8acc5e6cfc4bd00000000000000000000000000000000000000000000000000000003b69b3a25",
          "value": "0",
          "gas": 150000,
          "gasPrice": 100000000000,
          "nonce": "0x24093",
          "transactionIndex": 3,
          "from": "0xb3c4e9ca7c12a6277deb9eef2dece65953d1c864",
          "to": "0xe9ad69a67bbc025e6b70d04bf686f9dce48c6133",
          "status": true,
          "gasUsed": 41775,
          "contractAddress": "",
          "logs": []
        }
        "#;

        let tt = r#"
         {
           "blockNumber": 1353398,
           "hash": "0x22c8f97e6e3a496fa3c090f793cc36629c55f1817356b827e6c3a77cb9b609e4",
           "timestamp": 1603429238,
           "from": "0xe9ad69a67bbc025e6b70d04bf686f9dce48c6133",
           "to": "0xb3c4e9ca7c12a6277deb9eef2dece65953d1c864",
           "value": 15948528165,
           "contract": "0x500684ce0d4f04abedff3e54fcf8acc5e6cfc4bd",
           "method": "sweep",
           "status": true,
           "data": ""
         }
        "#;

        test_parse_transfers(tx, tt);
    }
    #[test]
    fn test_parse_mint() {
        let tx = r#"
        {
            "blockHash": "0xfa2e1258e47f70bacfe8febbaaa7cf5735d6a1bb617e177e7076d89df7dfcffb",
            "blockNumber": 448732,
            "hash": "0x7155a5a9d11597f3cf5ed3c33d1e2053f498640c844a6ff91d4207f19ead3cef",
            "timestamp": 1524567054,
            "input": "0x40c10f1900000000000000000000000059850f5afad839afc6ab20a434726ab9dbcce3dc00000000000000000000000000000000000000000000015775839c5aab6e7000",
            "value": "0",
            "gas": 90000,
            "gasPrice": 20000000000,
            "nonce": "0x43",
            "transactionIndex": 29,
            "from": "0xda904bc07fd95e39661941b3f6daded1b8a38c71",
            "to": "0xcf3222b7fda7a7563b9e1e6c966bead04ac23c36",
            "status": false,
            "gasUsed": 53592,
            "contractAddress": "",
            "logs": []
        }
        "#;

        let tt = r#"
         {
            "blockNumber": 448732,
            "hash": "0x7155a5a9d11597f3cf5ed3c33d1e2053f498640c844a6ff91d4207f19ead3cef",
            "timestamp": 1524567054,
            "from": "0x0000000000000000000000000000000000000000",
            "to": "0x59850f5afad839afc6ab20a434726ab9dbcce3dc",
            "value": 6335701000920000000000,
            "contract": "0xcf3222b7fda7a7563b9e1e6c966bead04ac23c36",
            "method": "mint",
            "status": false,
            "data": ""
        }
        "#;

        test_parse_transfers(tx, tt);
    }
}
