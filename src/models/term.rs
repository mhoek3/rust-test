use serde::{Deserialize, Serialize};
use std::fmt;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Term {
    pub id: u64,    // unsigned i suppose? , i64 signed?
    pub term_kind: u64,
    pub name: String,
    pub details: String,
}
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name: {}, details: {}", self.name, self.details)
    }
}