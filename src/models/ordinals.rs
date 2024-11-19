use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputResponse {
    pub address: String,
    pub inscriptions: Vec<String>,
    pub runes: HashMap<String, Rune>,
    pub transaction: String,
    pub value: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rune {
    pub amount: f64,
    pub divisibility: Option<u8>,
    pub symbol: Option<String>,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn deserize_test() {
        let output_response = r#"{
            "address": "bc1p90zah9c3hyywydpgnw0gcuk2pwwywj8u7hd0rhhr8kg0x3wl778s4d8h9t",
            "inscriptions": [
                "198ba1162cccd67fb7fd590db92b6e9f2bc052dce244d6d0ceaebb3bbc10e134i622"
            ],
            "runes": {},
            "transaction": "3de0c436d136abfb5f1ec1996d755331f25bf8e424743b1c21e2952fea8ef002",
            "value": 546
        }"#;
        let output_response: OutputResponse = serde_json::from_str(output_response).unwrap();
        assert_eq!(output_response.value, 546);
        assert_eq!(output_response.address, "bc1p90zah9c3hyywydpgnw0gcuk2pwwywj8u7hd0rhhr8kg0x3wl778s4d8h9t");
    }

    #[tokio::test]
    async fn empty_rune_test() {
        let output_response = r#"{
            "address": "bc1q80c2nv7ryjcw2a6uj2p6avd26rkcw4dc90a6mr",
            "indexed": false,
            "inscriptions": [],
            "runes": {},
            "sat_ranges": null,
            "script_pubkey": "OP_0 OP_PUSHBYTES_20 3bf0a9b3c324b0e5775c9283aeb1aad0ed8755b8",
            "spent": true,
            "transaction": "9967981989ae3c945cc2174d5ff7560af9d6d76a08ecc1eff2d854add40679ec",
            "value": 286588
        }"#;
        let output_response: OutputResponse = serde_json::from_str(output_response).unwrap();
        assert_eq!(output_response.value, 286588);
        assert_eq!(output_response.address, "bc1q80c2nv7ryjcw2a6uj2p6avd26rkcw4dc90a6mr");
    }

    #[tokio::test]
    async fn single_rune_test(){
        let output_response = r#"{
            "address": "bc1ppq9v5r7cu7w9nc408jyucvtpl2wnnw7kcdfu425z0f0e35f4h5yswtykl3",
            "indexed": true,
            "inscriptions": [],
            "runes": {
                "KODA•FLUFFINGTON": {
                    "amount": 7151041666667,
                    "divisibility": 8,
                    "symbol": "🐾"
                }
            },
            "sat_ranges": null,
            "script_pubkey": "OP_PUSHNUM_1 OP_PUSHBYTES_32 080aca0fd8e79c59e2af3c89cc3161fa9d39bbd6c353caaa827a5f98d135bd09",
            "spent": false,
            "transaction": "9967981989ae3c945cc2174d5ff7560af9d6d76a08ecc1eff2d854add40679ec",
            "value": 546
        }"#;

        let output_response: OutputResponse = serde_json::from_str(output_response).unwrap();
        assert_eq!(output_response.value, 546);
        assert_eq!(output_response.address, "bc1ppq9v5r7cu7w9nc408jyucvtpl2wnnw7kcdfu425z0f0e35f4h5yswtykl3");
        assert_eq!(output_response.runes["KODA•FLUFFINGTON"].amount, 7151041666667.0);
    }
}