use candid::{CandidType, Principal}; 
use ic_cdk::{query, update}; 
use serde::Deserialize; 
use std::cell::Cell; 

thread_local! {
    static COUNTER: Cell<u64> = Cell::new(0); 
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Counter{
    topic: String, 
    value: u64
}

#[derive(Clone, Debug,CandidType, Deserialize)]
struct Subscriber {
    topic: String 
}

#[update]
async fn setup_subscribe(publisher_id: Principal, topic: String) {
    let subscriber = Subscriber { topic }; // Creating a new instance of susbscriber struct with the topic that was passed to the function
    let _call_result: Result<(), _> = // Result from calling the subscriber function will be stored in the "_call_result". The underscore prefix "_" indicates that the result is not intentionally used after this
        ic_cdk::call(publisher_id, "subscribe", (subscriber, )).await; // This is where the inter canister call happens. It calls the subscribe function of the canister identified by "publisher_id"
        // It then passes the "subscriber" as an argument 
}

#[update] 
fn update_count(counter: Counter) {
    COUNTER.with(|c| {  // Uses "with" method of the counter to access its current value
        c.set(c.get() + counter.value); // It then takes a closure and the current value of the "COUNTER" is retrieved using the "get" method then increases it wuith the value of the counter passed into the function
        // The new value of the counter is then set using the "set" method
    }); 
}

#[query] 
fn get_count() -> u64 {
   COUNTER.with(|c| { // Uses the "with" method of the "COUNTER" to get its current value. The with method then takes a closure (an anonymous function) as an argument
     c.get() // This is inside the closure. The current value of the counter is retrieved using the "get" method and returned as a result of the whole function
   })
}

// Enabling candid export
ic_cdk::export_candid!(); 