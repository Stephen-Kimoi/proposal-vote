use candid::{CandidType, Encode, Principal}; 
use ic_cdk::{query, update}; 
use serde::Deserialize; 
use std::collections::BTreeMap; 
use std::cell::Cell; 
use ic_cdk::api::call::call; 
use ic_cdk::api::call::RejectionCode; 

#[derive(Clone, Debug, CandidType, Deserialize)] 
pub struct Proposal {
   title: String 
}

pub type Proposals = BTreeMap<(Principal, u64), Proposal>;

// Function that lists proposals 
#[update]
async fn list_proposals(canister_id: Principal) -> Result<Vec<((Principal, u64), Proposal)>, (RejectionCode, String)> {{
   let result: Result<Vec<((Principal, u64), Proposal)>, (RejectionCode, String)> = call(canister_id, "list_proposals", ()).await;
   result
}}

// Function for voting 
#[update] 
async fn vote(canister_id:Principal, proposal_id: u64, vote_value: bool) -> Result<(), String> {
   let args = Encode!((proposal_id, vote_value)).unwrap(); 
   let _: Result<(), _> = call(canister_id, "vote", args).await; 
   Ok(())
}

// Function that displays all proposals listed