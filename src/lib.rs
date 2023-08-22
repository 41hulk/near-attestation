use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use serde::{Deserialize, Serialize};

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
            listed_attestations: UnorderedMap::new(b"attestations".to_vec()),
        }
    }

    pub fn create_attestation(&mut self, payload: Payload) {
        let attestation = Attestation::from_payload(payload);
        self.listed_attestations
            .insert(&attestation.attestation_id, &attestation);
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
}

#[near_bindgen]
#[derive(Serialize, Deserialize, PanicOnDefault)]
pub struct Payload {
    attestation_id: String,
    project_id: String,
    description: String,
    evidence_uri: String,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, PanicOnDefault)]
pub struct Attestation {
    attestation_id: String,
    project_id: String,
    attestor: AccountId,
    description: String,
    evidence_uri: String,
    attested: u32,
}

#[near_bindgen]
impl Attestation {
    pub fn from_payload(payload: Payload) -> Self {
        Self {
            attestation_id: payload.attestation_id,
            project_id: payload.project_id,
            attestor: env::signer_account_id(),
            description: payload.description,
            evidence_uri: payload.evidence_uri,
            attested: 0,
        }
    }

    pub fn increment_attested_amount(&mut self) {
        self.attested = self.attested + 1;
    }
}
