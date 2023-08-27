use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, FunctionError, PanicOnDefault};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]

pub struct AttestationExplorer {
    listed_attestations: UnorderedMap<String, Attestation>,
}

#[near_bindgen]
impl AttestationExplorer {
    #[init]
    pub fn new() -> Self {
        Self {
            listed_attestations: UnorderedMap::new(
                SchemaList::AttestationList.try_to_vec().unwrap(),
            ),
        }
    }

    pub fn create_attestation(&mut self, payload: Payload) {
        let attestation = Attestation::from_payload(payload);
        self.listed_attestations
            .insert(&attestation.id, &attestation);
    }

    pub fn get_attestation(&self, id: &String) -> Option<Attestation> {
        self.listed_attestations.get(id)
    }

    pub fn get_attestations(&self) -> Vec<Attestation> {
        return self.listed_attestations.values_as_vector().to_vec();
    }

    pub fn get_last_attestation(&self) -> Option<Attestation> {
        self.listed_attestations.values().last()
    }

    #[handle_result]
    pub fn get_schema_value(&self, id: String, key: String) -> Result<String, ContractError> {
        let attestation = self
            .listed_attestations
            .get(&id)
            .ok_or(ContractError::AttestationNotFound { id })?;

        let schema: HashMap<String, String> = serde_json::from_str(&attestation.schema).unwrap();

        Ok(schema
            .get(&key)
            .ok_or(ContractError::KeyNotFound)?
            .to_string())
    }
}

#[derive(BorshSerialize, Debug, Error, FunctionError)]
pub enum ContractError {
    #[error("Key not found")]
    KeyNotFound,

    #[error("Attestation not found with id: {}", {id})]
    AttestationNotFound { id: String },
}
#[derive(Serialize, Deserialize)]
pub struct Payload {
    id: String,
    project_id: String,
    description: String,
    evidence_uri: String,
    schema: HashMap<String, String>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
pub struct Attestation {
    id: String,
    project_id: String,
    attestor: AccountId,
    description: String,
    evidence_uri: String,
    created_date: String,
    schema: String,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum SchemaList {
    AttestationList,
}

impl Attestation {
    pub fn from_payload(payload: Payload) -> Self {
        Self {
            id: payload.id,
            project_id: payload.project_id,
            attestor: env::signer_account_id(),
            description: payload.description,
            evidence_uri: payload.evidence_uri,
            created_date: env::block_timestamp_ms().to_string(),
            schema: serde_json::to_string(&payload.schema).unwrap(),
        }
    }
}
