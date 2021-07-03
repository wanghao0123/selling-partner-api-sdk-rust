/* 
 * Selling Partner API for Messaging
 *
 * With the Messaging API you can build applications that send messages to buyers. You can get a list of message types that are available for an order that you specify, then call an operation that sends a message to the buyer for that order. The Messaging API returns responses that are formed according to the <a href=https://tools.ietf.org/html/draft-kelly-json-hal-08>JSON Hypertext Application Language</a> (HAL) standard.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateConfirmServiceDetailsResponse : The response schema for the createConfirmServiceDetails operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateConfirmServiceDetailsResponse {
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl CreateConfirmServiceDetailsResponse {
  /// The response schema for the createConfirmServiceDetails operation.
  pub fn new() -> CreateConfirmServiceDetailsResponse {
    CreateConfirmServiceDetailsResponse {
      errors: None
    }
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> CreateConfirmServiceDetailsResponse {
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



