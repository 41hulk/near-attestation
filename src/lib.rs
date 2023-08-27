use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]

pub struct AttestationExplorer {
    listed_attestations: UnorderedMap<u32, Attestation>,
    count: u32,
}

#[near_bindgen]
impl AttestationExplorer {
    #[init]
    pub fn new() -> Self {
        Self {
            listed_attestations: UnorderedMap::new(b"attestations".to_vec()),
            count: 0,
        }
    }

    pub fn create_attestation(&mut self, payload: Payload) {
        let attestation = Attestation::from_payload(payload, self.count);
        self.listed_attestations
            .insert(&attestation.id, &attestation);
        self.count += 1;
    }

    pub fn get_attestation(&self, id: u32) -> Option<Attestation> {
        self.listed_attestations.get(&id)
    }

    pub fn get_attestations(&self) -> Vec<Attestation> {
        return self.listed_attestations.values_as_vector().to_vec();
    }

    pub fn upvote_attestation(&mut self, id: u32) -> bool {
        if let Some(mut attestation) = self.listed_attestations.get(&id) {
            attestation.upvotes += 1;
            self.listed_attestations.insert(&id, &attestation);
            true
        } else {
            false
        }
    }

    pub fn downvote_attestation(&mut self, id: u32) -> bool {
        if let Some(mut attestation) = self.listed_attestations.get(&id) {
            attestation.downvotes += 1;
            self.listed_attestations.insert(&id, &attestation);
            true
        } else {
            false
        }
    }

    pub fn get_last_attestation(&self) -> Option<Attestation> {
        self.listed_attestations.values().last()
    }
}

#[near_bindgen]
#[derive(Serialize, Deserialize, PanicOnDefault)]
pub struct Payload {
    name: String,
    project_id: String,
    description: String,
    image: String,
    evidence_uri: String,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, PanicOnDefault)]
pub struct Attestation {
    id: u32,
    name: String,
    project_id: String,
    attestor: AccountId,
    description: String,
    image: String,
    evidence_uri: String,
    upvotes: u32,
    downvotes: u32,
    created_at: u64,
}

#[near_bindgen]
impl Attestation {
    pub fn from_payload(payload: Payload, id: u32) -> Self {
        Self {
            id,
            name: payload.name,
            project_id: payload.project_id,
            attestor: env::signer_account_id(),
            description: payload.description,
            image: payload.image,
            evidence_uri: payload.evidence_uri,
            upvotes: 0,
            downvotes: 0,
            created_at: env::block_timestamp(),
        }
    }
}
