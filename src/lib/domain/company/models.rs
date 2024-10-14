use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Company {
  pub id: uuid::Uuid,
  pub name: Name,
}


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