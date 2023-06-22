use serde::{
    Serialize
};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Sample{
    pub id: u64,
    pub name: String,
    pub uuid: Uuid,
}

#[derive(Serialize)]
pub struct Samples{
    count: usize,
    samples: Vec<Sample>,
}