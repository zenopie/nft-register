use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::Config;

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct InstantiateMsg {
    pub owner: String,
    pub nft_contract: String,
    pub nft_contract_hash: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct PublicExtension {
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub attributes: Vec<Attribute>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct PrivateExtension {
    pub registered_at: String,
    pub wallet_address: String,
    pub description: String,
    pub platform: String,
    pub email: String,
    pub website: Option<String>,
    pub orcid_id: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct PublicMetadata {
    pub extension: PublicExtension,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct PrivateMetadata {
    pub extension: PrivateExtension,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig {
        config: Config
    },
    RegisterScientist {
        name: String,
        institution: String,
        specialization: String,
        description: String,
        email: String,
        image: Option<String>,
        website: Option<String>,
        orcid_id: Option<String>,
    },
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MigrateMsg {}

// SNIP721 Messages for cross-contract calls
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Snip721ExecuteMsg {
    MintNft {
        token_id: String,
        owner: String,
        public_metadata: Option<PublicMetadata>,
        private_metadata: Option<PrivateMetadata>,
        memo: Option<String>,
    },
}
