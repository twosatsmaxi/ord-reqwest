use std::fmt::Debug;

use bitcoin::blockdata::transaction::Transaction;
use ordinals::{Artifact, Edict, Etching, RuneId, Runestone, SpacedRune};

pub(crate) struct EtchingDetails {
    pub(crate) tx_id: String,
    pub(crate) rune_name: String,
    pub(crate) supply: Option<u128>,
    pub(crate) mintable: bool,
}
pub(crate) enum RuneTransaction {
    ETCHING(EtchingDetails),
    MINT(RuneId),
    TRANSFER(Vec<Edict>),
}
pub(crate) struct RuneTxDetails {
    pub(crate) tx_id: String,
    pub(crate) rune_tx: RuneTransaction,
}
// dervie copy for RuneTransactionDecoder
#[derive(Debug, Clone)]
pub(crate) struct RuneTransactionDecoder {}
impl RuneTransactionDecoder {
    pub(crate) fn new() -> Self {
        RuneTransactionDecoder {}
    }
    fn process_etching(tx_id: &String, etching: Etching) -> RuneTransaction {
        let rune_name = etching.rune.unwrap();
        let spacers = etching.spacers;
        let spaced_rune = if spacers.is_some() {
            Some(SpacedRune::new(rune_name, etching.spacers.unwrap()))
        } else {
            None
        };
        let supply = etching.supply();
        let rune_name = if spaced_rune.is_some() {
            spaced_rune.unwrap().to_string()
        } else {
            rune_name.to_string()
        };
        return RuneTransaction::ETCHING(EtchingDetails {
            tx_id: tx_id.to_string(),
            rune_name,
            supply,
            mintable: etching.terms.is_some(),
        });
    }
    fn process_runestone(tx_id: &String, rune: Runestone) -> RuneTransaction {
        if let Some(etching) = rune.etching {
            return Self::process_etching(tx_id, etching);
        }
        if let Some(mint) = rune.mint {
            return RuneTransaction::MINT(mint);
        }
        return RuneTransaction::TRANSFER(rune.edicts);
    }

    pub(crate) async fn decode_tx(self, transaction: &Transaction) -> Option<RuneTxDetails> {
        let rune_stone = Runestone::decipher(transaction);
        if rune_stone.is_none() {
            return None;
        }
        let txid = &transaction.txid().to_string();
        let rune_stone = rune_stone.unwrap();
        match rune_stone {
            Artifact::Runestone(rune) => {
                let rune_tx = RuneTransactionDecoder::process_runestone(txid, rune);
                // println!("Processed transaction {} in {:?}", txid, start.elapsed());
                return Some(RuneTxDetails {
                    tx_id: txid.to_string(),
                    rune_tx,
                });
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::runes::data::transaction::RUNE_BUY_TX;
use crate::runes::data::transaction::SIGNET_RUNE_TX;
use bitcoin::consensus::deserialize;
    use hex::decode as hex_decode;
    use crate::runes::data::transaction::{NON_RUNE_TX, RUNE_REVEAL_TX_HEX};


    use super::*;

    #[tokio::test]
    async fn test_decode_tx_runestone() {
        let tx_bytes = hex_decode(RUNE_REVEAL_TX_HEX).expect("Invalid hex string");
        let tx: Transaction = deserialize(&tx_bytes).expect("Failed to deserialize transaction");
        let rune_tx_details = RuneTransactionDecoder::new().decode_tx(&tx).await.unwrap();
        let runestone = rune_tx_details.rune_tx;
        match runestone {
            RuneTransaction::ETCHING(etching) => {
                assert_eq!(etching.rune_name, "HOOOOOOOOTERS");
            }
            _ => panic!("Expected etching"),
        }
    }
    #[tokio::test]
    async fn test_rune_tx_mint() {
        let mint_tx = "02000000000101e1abe66835908ec28bab86af8914ea85458993da13822e326a3bb1c159dfd0090200000000fdffffff0300000000000000000a6a5d0714c0a23314a30222020000000000002251206bda50e97f9e9107d24774e12099e6ef6fb11047f52949e0cae98ae4aa0c8f8676b9000000000000225120528996bda1de76858fdecd34168c331e12a64f415427ec060ae1df72b4aaaafb0140f4def7a7945dbfdeecc285163a794bd624c603261a02a6a87e9ccc6d56ee1c6b9fe8e48c0bd30e540ca17327d6b0be3b215f5b8edee32b824aa42add2a283b7c00000000";
        let tx_bytes = hex_decode(mint_tx).expect("Invalid hex string");
        let tx: Transaction = deserialize(&tx_bytes).expect("Failed to deserialize transaction");
        let rune_tx_details = RuneTransactionDecoder::new().decode_tx(&tx).await.unwrap();
        let runestone = rune_tx_details.rune_tx;
        match runestone {
            RuneTransaction::MINT(rune_id) => {
                assert_eq!(rune_id.block, 840000);
                assert_eq!(rune_id.tx, 291);
            }
            _ => panic!("Expected minted rune"),
        }
    }
    #[tokio::test]
    async fn test_decode_tx_non_rune_tx() {
        let tx_bytes = hex_decode(NON_RUNE_TX).expect("Invalid hex string");
        let tx: Transaction = deserialize(&tx_bytes).expect("Failed to deserialize transaction");
        let rune_tx_details = RuneTransactionDecoder::new().decode_tx(&tx).await;
        assert!(rune_tx_details.is_none());
    }

    #[tokio::test]
    async fn test_decode_signet_rune_tx() {
        let tx_bytes = hex_decode(SIGNET_RUNE_TX).expect("Invalid hex string");
        let tx: Transaction = deserialize(&tx_bytes).expect("Failed to deserialize transaction");
        let rune_tx_details = RuneTransactionDecoder::new().decode_tx(&tx).await.unwrap();
        let runestone = rune_tx_details.rune_tx;
        match runestone {
            RuneTransaction::ETCHING(etching) => {
                assert_eq!(etching.rune_name, "HOOOOOOOOTERS");
            }
            _ => panic!("Expected minted rune"),
        }
    }

    #[tokio::test]
    async fn test_decode_sell_tx() {
        let tx_bytes = hex_decode(RUNE_BUY_TX).expect("Invalid hex string");
        let tx: Transaction = deserialize(&tx_bytes).expect("Failed to deserialize transaction");
        let rune_tx_details = RuneTransactionDecoder::new().decode_tx(&tx).await.unwrap();
        let runestone = rune_tx_details.rune_tx;
        match runestone {
            RuneTransaction::TRANSFER(edicts) => {
                assert_eq!(edicts.len(), 1);
                assert_eq!(edicts.first().unwrap().id.block, 840010);
                assert_eq!(edicts.first().unwrap().id.tx, 4);
            }
            _ => panic!("Expected transfer rune"),
        }
    }
}
