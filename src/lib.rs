pub mod metadata;

use metadata::{Token, TokenMetadata};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  pub owner_id: AccountId,
  pub tokens: UnorderedMap<u64, Token>,
  pub token_per_token_id: LookupMap<String, Token>,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
  #[init]
  pub fn init() -> Self {
    Self {
      owner_id: env::signer_account_id(),
      tokens: UnorderedMap::new(b"tokens".to_vec()),
      token_per_token_id: LookupMap::new(b"token_per_token_id".to_vec()),
    }
  }

  pub fn create_nft_token_metadata(&mut self, metadata: TokenMetadata, token_id: String, price: Balance) -> Token {
    assert!(!self.token_per_token_id.contains_key(&token_id), "Token have exists");
    let metadata = TokenMetadata {
      title: metadata.title,
      description: metadata.description,
      media: metadata.media,
      media_hash: metadata.media_hash,
      copies: metadata.copies,
      issued_at: metadata.issued_at,
      expires_at: metadata.expires_at,
      starts_at: metadata.starts_at,
      updated_at: metadata.updated_at,
      extra: metadata.extra,
      reference: metadata.reference,
      reference_hash: metadata.reference_hash,
    };
    let token = Token { owner_id: env::signer_account_id(), metadata, token_id: token_id.clone(), price };
    let len = self.tokens.len() + 1;
    self.tokens.insert(&len, &token);
    self.token_per_token_id.insert(&token_id, &token);
    token
  }

  pub fn get_all_tokens(&self) -> Vec<Token> {
    let mut all_tokens = Vec::new();
    for i in 1..=self.tokens.len() {
      all_tokens.push(self.tokens.get(&i).unwrap());
    }
    all_tokens
  }

  // 7 - Cross Call > Front-end - Nextjs - BoS - Blockchain Operating System
  pub fn get_token_by_id(&self, token_id: String) -> Option<Token> {
    if let Some(token) = self.token_per_token_id.get(&token_id) {
      Some(token)
    } else {
      None
    }
  }

  // Futures
  #[payable]
  pub fn payment(&mut self, token_id: String) -> Promise {
    let mut token = self.token_per_token_id.get(&token_id).unwrap();
    let price = token.price;
    assert!(price == env::attached_deposit(), "Not equal the price");
    assert!(env::signer_account_id() != token.owner_id, "You are own of this token");

    token.owner_id = env::signer_account_id();
    self.token_per_token_id.insert(&token_id, &token);
    // insert to self.tokens
    Promise::new(token.owner_id).transfer(price)
  }

  // Fungible Token
  // Cross Call - Cross Contract
}
