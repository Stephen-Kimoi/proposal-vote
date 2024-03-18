use candid::{CandidType, Principal}; 
use ic_cdk::{query, update}; 
use serde::Deserialize; 
use std::collections::BTreeMap; 
use std::cell::Cell; 
use ic_cdk::api::call::call; 

#[derive(Clone, Debug, CandidType, Deserialize)] 
struct Proposal {
   title: String 
}

type Proposals = BTreeMap<(Principal, u64), Proposal>;

// Function that lists proposals 
// #[update] 
// async fn list_all_proposals(canister_id: Principal) -> Result<Proposals, > {
//    let result: Result<Proposals, _> = call(canister_id, "list_proposals", ()).await; 
//    result
// }

// Function that allows someone to vote 

// Function that displays all proposals listed