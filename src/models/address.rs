use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddressResponse {
    pub outputs: Vec<String>,
    pub inscriptions: Vec<String>,
    pub sat_balance: u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_response() {
        let json_data = r#"
                {
                  "outputs": ["ab"],
                  "inscriptions": ["jkjlk"],
                  "sat_balance": 809009
                }
                "#;
        let address_response: AddressResponse = serde_json::from_str(json_data).unwrap();
        assert_eq!(address_response.sat_balance, 809009);
        assert_eq!(address_response.outputs, vec!["ab"]);
        assert_eq!(address_response.inscriptions, vec!["jkjlk"]);
    }
}
