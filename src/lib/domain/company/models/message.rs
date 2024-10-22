use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct MessageProfessionalValidated {
    pub company_id: String,
    pub professional_id: String,
    pub message: String,
}
