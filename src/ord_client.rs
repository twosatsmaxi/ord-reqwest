use ordinals::RuneId;
use reqwest::{Error, Response};
use crate::data::rune_entry::RuneResponse;

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
