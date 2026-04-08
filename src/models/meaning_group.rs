use serde::{Deserialize, Serialize};
use std::fmt;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MeaningGroup {
    pub id: u64,    // unsigned i suppose? , i64 signed?
    pub name: String,
}
impl fmt::Display for MeaningGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name: {}", self.name)
    }
}
