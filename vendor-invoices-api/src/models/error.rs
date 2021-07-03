/* 
 * Selling Partner API for Retail Procurement Payments
 *
 * The Selling Partner API for Retail Procurement Payments provides programmatic access to vendors payments data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Error : Error response returned when the request is unsuccessful.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
  /// An error code that identifies the type of error that occurred.
  #[serde(rename = "code")]
  code: String,
  /// A message that describes the error condition.
  #[serde(rename = "message")]
  message: String,
  /// Additional details that can help the caller understand or fix the issue.
  #[serde(rename = "details")]
  details: Option<String>
}

impl Error {
  /// Error response returned when the request is unsuccessful.
  pub fn new(code: String, message: String) -> Error {
    Error {
      code: code,
      message: message,
      details: None
    }
  }

  pub fn set_code(&mut self, code: String) {
    self.code = code;
  }

  pub fn with_code(mut self, code: String) -> Error {
    self.code = code;
    self
  }

  pub fn code(&self) -> &String {
    &self.code
  }


  pub fn set_message(&mut self, message: String) {
    self.message = message;
  }

  pub fn with_message(mut self, message: String) -> Error {
    self.message = message;
    self
  }

  pub fn message(&self) -> &String {
    &self.message
  }


  pub fn set_details(&mut self, details: String) {
    self.details = Some(details);
  }

  pub fn with_details(mut self, details: String) -> Error {
    self.details = Some(details);
    self
  }

  pub fn details(&self) -> Option<&String> {
    self.details.as_ref()
  }

  pub fn reset_details(&mut self) {
    self.details = None;
  }

}



