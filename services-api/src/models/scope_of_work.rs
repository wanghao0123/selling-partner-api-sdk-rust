/* 
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ScopeOfWork : The scope of work for the order.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScopeOfWork {
  /// The Amazon Standard Identification Number (ASIN) of the service job.
  #[serde(rename = "asin")]
  asin: Option<String>,
  /// The title of the service job.
  #[serde(rename = "title")]
  title: Option<String>,
  /// The number of service jobs.
  #[serde(rename = "quantity")]
  quantity: Option<i32>,
  /// A list of skills required to perform the job.
  #[serde(rename = "requiredSkills")]
  required_skills: Option<Vec<String>>
}

impl ScopeOfWork {
  /// The scope of work for the order.
  pub fn new() -> ScopeOfWork {
    ScopeOfWork {
      asin: None,
      title: None,
      quantity: None,
      required_skills: None
    }
  }

  pub fn set_asin(&mut self, asin: String) {
    self.asin = Some(asin);
  }

  pub fn with_asin(mut self, asin: String) -> ScopeOfWork {
    self.asin = Some(asin);
    self
  }

  pub fn asin(&self) -> Option<&String> {
    self.asin.as_ref()
  }

  pub fn reset_asin(&mut self) {
    self.asin = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> ScopeOfWork {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

  pub fn set_quantity(&mut self, quantity: i32) {
    self.quantity = Some(quantity);
  }

  pub fn with_quantity(mut self, quantity: i32) -> ScopeOfWork {
    self.quantity = Some(quantity);
    self
  }

  pub fn quantity(&self) -> Option<&i32> {
    self.quantity.as_ref()
  }

  pub fn reset_quantity(&mut self) {
    self.quantity = None;
  }

  pub fn set_required_skills(&mut self, required_skills: Vec<String>) {
    self.required_skills = Some(required_skills);
  }

  pub fn with_required_skills(mut self, required_skills: Vec<String>) -> ScopeOfWork {
    self.required_skills = Some(required_skills);
    self
  }

  pub fn required_skills(&self) -> Option<&Vec<String>> {
    self.required_skills.as_ref()
  }

  pub fn reset_required_skills(&mut self) {
    self.required_skills = None;
  }

}



