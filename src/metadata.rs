use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  serde::{Deserialize, Serialize},
  AccountId, Balance,
};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
  pub title: Option<String>,
  pub description: Option<String>,
  pub media: Option<String>,
  pub media_hash: Option<String>,
  pub copies: Option<u64>,
  pub issued_at: Option<u64>,
  pub expires_at: Option<u64>,
  pub starts_at: Option<u64>,
  pub updated_at: Option<u64>,
  pub extra: Option<String>,
  pub reference: Option<String>,
  pub reference_hash: Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
  pub owner_id: AccountId,
  pub token_id: String,
  // TODO: Thử thay đổi Balance thành U128
  pub price: Balance,
  pub metadata: TokenMetadata,
}
