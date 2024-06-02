use ordinals::{Edict, RuneId};

pub struct EtchingDetails {
    pub tx_id: String,
    pub rune_name: String,
    pub supply: Option<u128>,
    pub mintable: bool,
}
pub enum RuneTransaction {
    ETCHING(EtchingDetails),
    MINT(RuneId),
    TRANSFER(Vec<Edict>),
}
pub struct RuneTxDetails {
    pub tx_id: String,
    pub rune_tx: RuneTransaction,
}