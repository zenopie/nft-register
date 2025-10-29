use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use secret_toolkit::storage::Item;
use cosmwasm_std::Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub nft_contract: Addr,
    pub nft_contract_hash: String,
}

// Storage
pub const CONFIG: Item<Config> = Item::new(b"config");
