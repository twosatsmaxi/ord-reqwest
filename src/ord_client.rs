use bitcoin::OutPoint;
use ordinals::RuneId;
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};
use crate::data::rune_entry::RuneResponse;
use crate::models::address::AddressResponse;
use crate::models::ordinals::OutputResponse;

pub struct OrdClient {
    client: reqwest::Client,
    base_api_url: String,
    pub base_public_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InscriptionResponse {
    pub address: String,
    pub id: String,
}
impl OrdClient {
    pub fn new() -> Self {
        let ord_base_url =
            std::env::var("ORD_BASE_URL").unwrap_or("http://192.168.1.105:4000".to_string());
        let ord_public_url =
            std::env::var("ORD_PUBLIC_URL").unwrap_or("https://ordinals.com".to_string());
        let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(10)).build().unwrap();
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
        rune_response
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
    pub async fn fetch_output(&self, out_point: OutPoint) -> OutputResponse {
        // fetch output details from ord api using ord base url /output/{tx_id}:{vout}
        let output_url = format!("{}/output/{}:{}", self.base_api_url, out_point.txid, out_point.vout);
        // get the response and parse it using serde
        let api_response = self.do_api_call(&output_url).await;
        // get the output details from the response serde json it to OutputResponse and get the output details use serdejson
        let output_response =
            serde_json::from_str::<OutputResponse>(&api_response.unwrap().text().await.unwrap())
                .unwrap();
        return output_response;
    }
    pub async fn get_address(&self, address: &str) -> AddressResponse {
        // fetch address details from ord api using ord base url /address/{address}
        let address_url = format!("{}/address/{}", self.base_api_url, address);
        // get the response and parse it using serde
        let api_response = self.do_api_call(&address_url).await;
        // get the address details from the response serde json it to AddressResponse and get the address details use serdejson
        let address_response = api_response.unwrap().text().await.unwrap();
        let address_response: AddressResponse = serde_json::from_str(&address_response).unwrap();
        address_response
    }

    pub async fn get_inscription(&self, inscription_id: &str) -> InscriptionResponse {
        // fetch inscription details from ord api using ord base url /inscription/{inscription_id}
        let inscription_url = format!("{}/inscription/{}", self.base_api_url, inscription_id);
        // get the response and parse it using serde
        let api_response = self.do_api_call(&inscription_url).await.unwrap();
        // parse the JSON response to InscriptionResponse
        let inscription_text = api_response.text().await.unwrap();
        let inscription_response: InscriptionResponse = serde_json::from_str(&inscription_text).unwrap();
        inscription_response
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use bitcoin::{OutPoint, Txid};
    use crate::models::address::AddressResponse;
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn fetch_output() {
        let client = OrdClient::new();
        let out_point = OutPoint {
            txid: Txid::from_str("3de0c436d136abfb5f1ec1996d755331f25bf8e424743b1c21e2952fea8ef002").unwrap(),
            vout: 1
        };
        let output_response = client.fetch_output(out_point).await;
        assert_eq!(output_response.value, 546);
        assert_eq!(output_response.address, "bc1p90zah9c3hyywydpgnw0gcuk2pwwywj8u7hd0rhhr8kg0x3wl778s4d8h9t");
    }

    #[tokio::test]
    #[ignore]
    async fn fetch_address_details() {
        let client = OrdClient::new();
        let address = "bc1pk244ecgfnyurjdj43qh9ha95laff32aa5w7fmscjtt93fkresymqpf8rgz";
        let address_response: AddressResponse = client.get_address(address).await;
        assert!(address_response.inscriptions.len() > 0);
    }

    #[tokio::test]
    #[ignore]
    async fn fetch_latest_block_height() {
        let client = OrdClient::new();
        let block_height = client.fetch_latest_block_height().await;
        assert!(block_height > 0);
    }

    #[tokio::test]
    #[ignore]
    async fn fetch_inscription_details() {
        let client = OrdClient::new();
        let inscription_id = "9f7e2a095aa6773b4be7673f447fb2285f85fefb845e5d5cd06a38e2a1d0ae5di0";
        let inscription_details = client.get_inscription(inscription_id).await;
        let details = inscription_details;
        assert_eq!(details.id, inscription_id);
    }
}
