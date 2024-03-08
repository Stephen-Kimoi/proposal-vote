use candid::{CandidType, Deserialize, Principal}; 
use ic_cdk::{ update, query }; 
use std::cell::RefCell; 
use std::collections::BTreeMap; 
use std::sync::atomic::{AtomicU64, Ordering}; 

type Proposals = BTreeMap<(Principal, u64), Proposal>; 
type Votes = BTreeMap<(Principal, u64), bool>; 

thread_local! { 
    static PROPOSALS: RefCell<Proposals> = RefCell::default(); 
    static VOTES: RefCell<Votes> = RefCell::default(); 
}

static PROPOSAL_ID: AtomicU64 = AtomicU64::new(0); 

#[derive(Clone, Debug, CandidType, Deserialize)] 
struct Proposal {
   title: String 
}

#[update] 
fn propose(proposal: Proposal) {
    let proposer_id = ic_cdk::caller(); 
    let proposal_id = PROPOSAL_ID.fetch_add(1, Ordering::SeqCst); 
    PROPOSALS.with(|proposals|{
       proposals.borrow_mut().insert((proposer_id, proposal_id), proposal); 
    }); 
}

#[update] 
fn vote(proposal_id: u64, vote_value: bool) {
    let voter_id = ic_cdk::caller(); 
    VOTES.with(|votes| {
        votes.borrow_mut().insert((voter_id, proposal_id), vote_value); 
    }); 
}

#[query] 
fn list_proposals() -> Proposals {
   PROPOSALS.with(|proposals| proposals.borrow().clone())
} 

ic_cdk::export_candid!(); 
