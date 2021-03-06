/* 
 * Selling Partner API for Product Fees
 *
 * The Selling Partner API for Product Fees lets you programmatically retrieve estimated fees for a product. You can then account for those fees in your pricing.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FeesEstimateError : An unexpected error occurred during this operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeesEstimateError {
  /// An error type, identifying either the receiver or the sender as the originator of the error.
  #[serde(rename = "Type")]
  _type: String,
  /// An error code that identifies the type of error that occurred.
  #[serde(rename = "Code")]
  code: String,
  /// A message that describes the error condition.
  #[serde(rename = "Message")]
  message: String,
  #[serde(rename = "Detail")]
  detail: ::models::FeesEstimateErrorDetail
}

impl FeesEstimateError {
  /// An unexpected error occurred during this operation.
  pub fn new(_type: String, code: String, message: String, detail: ::models::FeesEstimateErrorDetail) -> FeesEstimateError {
    FeesEstimateError {
      _type: _type,
      code: code,
      message: message,
      detail: detail
    }
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> FeesEstimateError {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


  pub fn set_code(&mut self, code: String) {
    self.code = code;
  }

  pub fn with_code(mut self, code: String) -> FeesEstimateError {
    self.code = code;
    self
  }

  pub fn code(&self) -> &String {
    &self.code
  }


  pub fn set_message(&mut self, message: String) {
    self.message = message;
  }

  pub fn with_message(mut self, message: String) -> FeesEstimateError {
    self.message = message;
    self
  }

  pub fn message(&self) -> &String {
    &self.message
  }


  pub fn set_detail(&mut self, detail: ::models::FeesEstimateErrorDetail) {
    self.detail = detail;
  }

  pub fn with_detail(mut self, detail: ::models::FeesEstimateErrorDetail) -> FeesEstimateError {
    self.detail = detail;
    self
  }

  pub fn detail(&self) -> &::models::FeesEstimateErrorDetail {
    &self.detail
  }


}



