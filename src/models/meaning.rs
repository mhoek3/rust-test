use serde::{Deserialize, Serialize};
use std::fmt;
use sqlx::FromRow;

#[derive(Debug, Deserialize, FromRow)]
pub struct CreateMeaning {
    pub id: u64,
    pub group_id: u64,
    pub name: String,
    pub details: String,
}
impl fmt::Display for CreateMeaning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name: {}, details: {}", self.name, self.details)
    }
}

#[derive(Debug, Serialize, FromRow)]
pub struct Meaning {
    pub id: u64,    // unsigned i suppose? , i64 signed?
    pub group_id: u64,
    pub name: String,
    pub details: String,
}