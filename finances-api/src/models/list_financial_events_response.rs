/* 
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ListFinancialEventsResponse : The response schema for the listFinancialEvents operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListFinancialEventsResponse {
  /// The payload for the listFinancialEvents operation.
  #[serde(rename = "payload")]
  payload: Option<::models::ListFinancialEventsPayload>,
  /// One or more unexpected errors occurred during the listFinancialEvents operation.
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl ListFinancialEventsResponse {
  /// The response schema for the listFinancialEvents operation.
  pub fn new() -> ListFinancialEventsResponse {
    ListFinancialEventsResponse {
      payload: None,
      errors: None
    }
  }

  pub fn set_payload(&mut self, payload: ::models::ListFinancialEventsPayload) {
    self.payload = Some(payload);
  }

  pub fn with_payload(mut self, payload: ::models::ListFinancialEventsPayload) -> ListFinancialEventsResponse {
    self.payload = Some(payload);
    self
  }

  pub fn payload(&self) -> Option<&::models::ListFinancialEventsPayload> {
    self.payload.as_ref()
  }

  pub fn reset_payload(&mut self) {
    self.payload = None;
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> ListFinancialEventsResponse {
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



