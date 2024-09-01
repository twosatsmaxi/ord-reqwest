use std::fmt;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde::de::Visitor;

fn string_or_number<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrNumberVisitor;

    impl<'de> Visitor<'de> for StringOrNumberVisitor {
        type Value = f64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or a number")
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            value.parse::<f64>().map_err(E::custom)
        }
    }

    deserializer.deserialize_any(StringOrNumberVisitor)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RuneBalance {
    pub rune_name: String,
    #[serde(deserialize_with = "string_or_number")]
    pub balance: f64,
    pub rune_symbol: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AddressResponse {
    pub outputs: Vec<String>,
    pub inscriptions: Vec<String>,
    pub sat_balance: u64,
    pub runes_balances: Vec<RuneBalance>,
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
                  "sat_balance": 809009,
                  "runes_balances": [
                    [
                      "SAIKO•HAMSTER",
                      "10150",
                      "🐹"
                    ]
                  ]
                }
                "#;
        let address_response: AddressResponse = serde_json::from_str(json_data).unwrap();
        assert_eq!(address_response.sat_balance, 809009);
        assert_eq!(address_response.outputs, vec!["ab"]);
        assert_eq!(address_response.inscriptions, vec!["jkjlk"]);
        assert_eq!(address_response.runes_balances[0].rune_name, "SAIKO•HAMSTER");
        assert_eq!(address_response.runes_balances[0].balance, 10150.0);
        assert_eq!(address_response.runes_balances[0].rune_symbol, "🐹");
    }
}