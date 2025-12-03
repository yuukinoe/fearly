use juli_orm::*;
use juli_orm_core::ForeignKey;
use serde::{Deserialize, Serialize};
// use std::fmt;
use surrealdb::sql::Thing;

// type FileField = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[juli_register_fields]
#[juli_model()]
pub struct User {
    pub id: Thing,
    pub name: String,
    pub password: String,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
    pub published: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub is_superuser: bool,
    pub is_staff: bool,
    pub is_active: bool,
    pub is_banned: bool,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[juli_register_fields]
#[juli_model()]
pub struct Sessions {
    pub id: Thing,
    pub user_id: Thing,
    pub session_id: String,
    pub expiration: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[juli_register_fields]
#[juli_model(user="User")]
pub struct Fear {
    pub id: Thing,
    pub user: ForeignKey<User>,
    pub title: String,
    pub content: String,
    pub reaction: String,
    pub emotion:  i8,
    pub datetime: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
