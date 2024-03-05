use std::cell::{Ref, RefCell};

use candid::types::number::Nat;
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
thread_local! {
    static COUNTER : RefCell<Nat> = RefCell::new(Nat::from(0_u32));
}

//Get the value of the counter for this we will use ic_sdk query
#[ic_cdk::query]
fn get()-> Nat{
    COUNTER.with(|counter| (*counter.borrow()).clone())
}

//Setting the value of the counter we use ic_sdk::update
#[ic_cdk::update]
fn set(m:Nat){
    COUNTER.with(|counter| (*counter.borrow_mut())= m)
}

//Incrementing the value of the counter 
#[ic_cdk::update]
fn increment() {
    COUNTER.with(|counter| *counter.borrow_mut()+=1_u32)
}

