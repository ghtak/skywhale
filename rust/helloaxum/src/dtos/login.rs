use serde::{
    Serialize, Deserialize
};

#[derive(Debug,Deserialize)]
pub struct LoginPayload{
    email: String,
    password: String
}