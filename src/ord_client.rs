use ordinals::{RuneId, Terms};
use reqwest::{Error, Response};

#[derive(serde::Deserialize)]
pub struct RuneEntry {
    pub spaced_rune: String,
    pub mints: u128,
    pub premine: u128,
    pub divisibility: u16,
    pub number: u128,
    pub terms: Option<Terms>,
}

impl RuneEntry {
    pub fn remaining_mints(&self) -> u128 {
        if self.terms.is_none() {
            return 0;
        }
        if let Some(cap) = self.terms.unwrap().cap {
            return cap - self.mints;
        }
        return 0;
    }
    pub fn premine_percentage(&self) -> f32 {
        if self.premine == 0 {
            return 0.0;
        }
        if self.terms.is_none() {
            return 0.0;
        }
        // normalize premine by dividing it to 10 ^ divisibility
        let denominator = 10_i32.pow(self.divisibility.into());

        let premine_normalized = if denominator != 0 {
            self.premine
                .div_ceil(10_i32.pow(self.divisibility.into()) as u128)
        } else {
            self.premine
        };
        let terms = self.terms.unwrap();
        let total_mints_supply = terms.amount.unwrap() * self.mints;
        let total_mints_normalized =
            total_mints_supply.div_ceil(10_i32.pow(self.divisibility.into()) as u128);
        let circulating_supply = total_mints_normalized + premine_normalized;
        let premine_percentage = (premine_normalized * 100) as f32 / circulating_supply as f32;
        // round to 2 decimal places
        return (premine_percentage * 100.0).round() / 100.0;
    }
}
#[derive(serde::Deserialize)]
pub struct RuneResponse {
    pub entry: RuneEntry,
    pub parent: Option<String>,
}

pub struct OrdClient {
    client: reqwest::Client,
    base_api_url: String,
    pub base_public_url: String,
}

impl OrdClient {
    pub fn new() -> Self {
        let ord_base_url =
            std::env::var("ORD_BASE_URL").unwrap_or("http://192.168.29.108:4000".to_string());
        let ord_public_url =
            std::env::var("ORD_PUBLIC_URL").unwrap_or("https://ordinals.com".to_string());
        let client = reqwest::Client::new();
        OrdClient {
            client,
            base_api_url: ord_base_url,
            base_public_url: ord_public_url,
        }
    }

    async fn do_api_call(&self, url: &str) -> Result<Response, Error> {
        // loop until we get a response from the api
        loop {
            let response = self
                .client
                .get(url)
                .header("accept", "application/json")
                .send()
                .await;
            if response.is_ok() {
                return response;
            }
        }
    }

    pub async fn fetch_rune_details(&self, rune_id: RuneId) -> RuneResponse {
        // fetch rune details from ord api using ord base url /rune/{rune_id}
        let rune_url = format!("{}/rune/{}", self.base_api_url, rune_id);
        // get the response and parse it using serde
        let api_response = self.do_api_call(&rune_url).await;
        // get the spaced rune from the response serde json it to RuneResponse and get the spaced rune use serdejson
        let rune_response =
            serde_json::from_str::<RuneResponse>(&api_response.unwrap().text().await.unwrap())
                .unwrap();
        return rune_response;
    }

    pub async fn fetch_latest_block_height(&self) -> u64 {
        // fetch latest block height from ord api using ord base url /block_height
        let block_height_url = format!("{}/blockheight", self.base_api_url);
        // get the response and parse it using serde
        let api_response = self.do_api_call(&block_height_url).await;
        // get the block height from the response serde json it to u128 and get the block height use serdejson
        let block_height =
            serde_json::from_str::<u64>(&api_response.unwrap().text().await.unwrap()).unwrap();
        return block_height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_premine_percentage() {
        let rune_entry = RuneEntry {
            divisibility: 2,
            spaced_rune: "Rune".to_string(),
            mints: 0,
            premine: 100,
            number: 1,
            terms: Some(Terms {
                amount: Some(1000),
                cap: Some(1000),
                ..Default::default()
            }),
        };
        assert_eq!(rune_entry.premine_percentage(), 100.0);
    }

    #[test]
    fn test_premine_percentage_nakamato() {
        let rune_entry = RuneEntry {
            divisibility: 0,
            spaced_rune: "Rune".to_string(),
            mints: 168000,
            premine: 420000000000000,
            number: 6,
            terms: Some(Terms {
                amount: Some(10000000000),
                cap: Some(168000),
                ..Default::default()
            }),
        };
        assert_eq!(rune_entry.premine_percentage(), 20.0);
    }

    #[test]
    fn test_premine_for_fehu() {
        let rune_entry = RuneEntry {
            spaced_rune: "Fehu".to_string(),
            mints: 452105,
            divisibility: 2,
            premine: 11000000000,
            number: 1,
            terms: Some(Terms {
                amount: Some(100),
                cap: Some(1111111),
                ..Default::default()
            }),
        };
        assert_eq!(rune_entry.premine_percentage(), 99.59);
    }
}
