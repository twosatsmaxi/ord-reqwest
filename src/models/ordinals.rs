use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputResponse {
    pub address: String,
    pub inscriptions: Vec<String>,
    pub transaction: String,
    pub value: u64,
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
            "transaction": "3de0c436d136abfb5f1ec1996d755331f25bf8e424743b1c21e2952fea8ef002",
            "value": 546
        }"#;
        let output_response: OutputResponse = serde_json::from_str(output_response).unwrap();
        assert_eq!(output_response.value, 546);
        assert_eq!(output_response.address, "bc1p90zah9c3hyywydpgnw0gcuk2pwwywj8u7hd0rhhr8kg0x3wl778s4d8h9t");
    }
}