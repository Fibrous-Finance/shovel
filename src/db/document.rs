use crate::common::types::CairoUint256;
use mongodb::bson::doc;
use serde::{self, Deserialize, Serialize};
use starknet::core::types::FieldElement;

#[derive(Debug, Deserialize, Serialize)]
pub struct AddressAtBlock {
    pub address: FieldElement,
    pub block: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractMetadata {
    pub _id: FieldElement,
    pub name: String,
    pub symbol: String,
}

impl ContractMetadata {
    pub fn new(contract_address: FieldElement, name: String, symbol: String) -> Self {
        Self { _id: contract_address, name, symbol }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Erc721Id {
    pub contract_address: FieldElement,
    pub token_id: CairoUint256,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Erc721 {
    pub _id: Erc721Id,
    pub owner: FieldElement,
    pub previous_owners: Vec<AddressAtBlock>,
    pub token_uri: String,
    pub metadata: TokenMetadata,
}

impl Erc721 {
    pub fn new(
        contract_address: FieldElement,
        token_id: CairoUint256,
        owner: FieldElement,
        token_uri: String,
        metadata: TokenMetadata,
    ) -> Self {
        Self {
            _id: Erc721Id { contract_address, token_id },
            owner,
            previous_owners: vec![],
            token_uri,
            metadata,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Erc1155MetadataId {
    pub contract_address: FieldElement,
    pub token_id: CairoUint256,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Erc1155Metadata {
    pub _id: Erc1155MetadataId,
    pub token_uri: String,
    pub metadata: TokenMetadata,
}

#[allow(unused)]
impl Erc1155Metadata {
    pub fn new(
        contract_address: FieldElement,
        token_id: CairoUint256,
        token_uri: String,
        metadata: TokenMetadata,
    ) -> Self {
        Self { _id: Erc1155MetadataId { contract_address, token_id }, token_uri, metadata }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Erc1155BalanceId {
    pub contract_address: FieldElement,
    pub token_id: CairoUint256,
    pub owner: FieldElement,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Erc1155Balance {
    pub _id: Erc1155BalanceId,
    pub balance: CairoUint256,
}

impl Erc1155Balance {
    pub fn new(
        contract_address: FieldElement,
        token_id: CairoUint256,
        owner: FieldElement,
        balance: CairoUint256,
    ) -> Self {
        Self { _id: Erc1155BalanceId { contract_address, token_id, owner }, balance }
    }
}

pub enum MetadataType<'a> {
    Http(&'a str),
    Ipfs(&'a str),
    OnChain(&'a str),
}

#[derive(Debug, Deserialize, Serialize)]
enum DisplayType {
    Number,
    BoostPercentage,
    BoostNumber,
    Date,
}

#[derive(Debug, Deserialize, Serialize)]
enum AttributeValue {
    Int(i128),
    Float(f64),
    String(String),
}

#[derive(Debug, Deserialize, Serialize)]
struct Attribute {
    display_type: Option<DisplayType>,
    trait_type: Option<String>,
    value: AttributeValue,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenMetadata {
    image: Option<String>,
    image_data: Option<String>,
    external_url: Option<String>,
    description: Option<String>,
    name: Option<String>,
    attributes: Option<Vec<Attribute>>,
    background_color: Option<String>,
    animation_url: Option<String>,
    youtube_url: Option<String>,
}

impl TokenMetadata {
    pub const EMPTY: Self = Self {
        name: None,
        description: None,
        image: None,
        image_data: None,
        external_url: None,
        animation_url: None,
        attributes: None,
        background_color: None,
        youtube_url: None,
    };
}
