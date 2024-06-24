use fake::Dummy;
use serde::{Deserialize, Serialize};
use fake::faker::{company::en::*, name::en::*, boolean::en::*};

#[derive(Debug, Clone, Dummy, Serialize, Deserialize)]
pub struct User {
    #[dummy(faker = "100..")]
    pub id: u32,
    #[dummy(faker = "Name()")]
    pub name: String,
    #[dummy(faker = "Boolean(70)")]
    pub is_active: bool,
    #[dummy(faker = "CompanyName()")]
    pub company: String,
}