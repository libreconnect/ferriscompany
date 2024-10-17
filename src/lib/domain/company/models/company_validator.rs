use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct CreateCompany {
    pub name: String,
    pub city: String,
    pub country: String,
    pub email: String,
    pub phone: String,
    pub zip_code: String,
    pub address: String,
    pub national_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct AttachProfessionalInCompany {
    pub professional_id: String,
    pub role: String,
    pub permissions: Vec<String>,
}
