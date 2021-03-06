/* 
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetServiceJobsResponse : Response schema for GetJobs operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetServiceJobsResponse {
  /// The payload for the GetJobs operation.
  #[serde(rename = "payload")]
  payload: Option<::models::JobListing>,
  /// An unexpected condition occurred during the GetServiceJobs operation.
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl GetServiceJobsResponse {
  /// Response schema for GetJobs operation.
  pub fn new() -> GetServiceJobsResponse {
    GetServiceJobsResponse {
      payload: None,
      errors: None
    }
  }

  pub fn set_payload(&mut self, payload: ::models::JobListing) {
    self.payload = Some(payload);
  }

  pub fn with_payload(mut self, payload: ::models::JobListing) -> GetServiceJobsResponse {
    self.payload = Some(payload);
    self
  }

  pub fn payload(&self) -> Option<&::models::JobListing> {
    self.payload.as_ref()
  }

  pub fn reset_payload(&mut self) {
    self.payload = None;
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> GetServiceJobsResponse {
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



