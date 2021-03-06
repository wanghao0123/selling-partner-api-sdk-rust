/* 
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CompleteServiceJobByServiceJobIdResponse : Response schema for CompleteServiceJobByServiceJobId operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteServiceJobByServiceJobIdResponse {
  /// Encountered errors for the CompleteServiceJobByServiceJobId operation.
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl CompleteServiceJobByServiceJobIdResponse {
  /// Response schema for CompleteServiceJobByServiceJobId operation.
  pub fn new() -> CompleteServiceJobByServiceJobIdResponse {
    CompleteServiceJobByServiceJobIdResponse {
      errors: None
    }
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> CompleteServiceJobByServiceJobIdResponse {
    self.errors = Some(errors);
    self
  }

  pub fn errors(&self) -> Option<&::models::ErrorList> {
    self.errors.as_ref()
  }

  pub fn reset_errors(&mut self) {
    self.errors = None;
  }

}



