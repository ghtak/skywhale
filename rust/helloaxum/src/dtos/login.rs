use serde::{
    Serialize, Deserialize
};

#[derive(Deserialize)]
pub struct LoginReq{
    email: String,
    password: String
}