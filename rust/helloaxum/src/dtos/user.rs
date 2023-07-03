use serde::{
    Serialize
};
use uuid::Uuid;

#[derive(Serialize)]
pub struct User{
    pub id: u64,
    pub name: String,
    pub uuid: Uuid,
}

#[derive(Serialize)]
pub struct Users{
    pub count: usize,
    pub users: Vec<User>,
}