use serde::{Deserialize, Serialize};

use crate::models::Fear;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserLoginData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GenericResponse {
    Message(String),
    Data(serde_json::Value),
    VecData(Vec<serde_json::Value>),
    FearData(Fear),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Response {
    pub message: GenericResponse,
    pub status: u16,
}



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditFear {
    pub title: String,
    pub content: String,
    pub emotion: i8,
    pub reaction: String,
    pub datetime: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
