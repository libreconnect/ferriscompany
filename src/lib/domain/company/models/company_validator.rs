#[derive(Debug, Clone)]
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
