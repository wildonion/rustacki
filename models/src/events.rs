


use serde::{Serialize, Deserialize};
use crate::*;


#[derive(Serialize, Deserialize)]
pub struct EventInfoQuery{
    pub etype: Type,
    pub eid: String,
}

#[derive(Serialize, Deserialize)]
pub enum Type{
    T1,
    T2,
    T3
}

// events table 
//     - id
//     - manager_id
//     - type
//     - ...
//     - created_at
//     - expired_at
//     - is_locked
//     - is_expired
//     - finished_at
//     - is_disabled
//     - started_at