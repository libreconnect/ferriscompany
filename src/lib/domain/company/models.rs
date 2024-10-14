use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Company {
    pub id: uuid::Uuid,
    pub name: Name,
}

impl Company {
    pub fn new(value: &str) -> Company {
        let id = uuid::Uuid::new_v4();
        let name = Name::new(value).unwrap();
        Company { id, name }
    }
}

#[derive(Debug, Clone, Error)]
pub enum CompanyError {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name(String);

#[derive(Clone, Debug, Error)]
#[error("Name cannot be empty")]
pub struct NameEmptyError;

impl Name {
    pub fn new(value: &str) -> Result<Name, NameEmptyError> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            Err(NameEmptyError)
        } else {
            Ok(Name(trimmed.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
