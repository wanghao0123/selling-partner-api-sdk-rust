/* 
 * Selling Partner API for Messaging
 *
 * With the Messaging API you can build applications that send messages to buyers. You can get a list of message types that are available for an order that you specify, then call an operation that sends a message to the buyer for that order. The Messaging API returns responses that are formed according to the <a href=https://tools.ietf.org/html/draft-kelly-json-hal-08>JSON Hypertext Application Language</a> (HAL) standard.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetMessagingActionResponse : Describes a messaging action that can be taken for an order. Provides a JSON Hypertext Application Language (HAL) link to the JSON schema document that describes the expected input.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMessagingActionResponse {
  #[serde(rename = "_links")]
  _links: Option<Value>,
  #[serde(rename = "_embedded")]
  _embedded: Option<Value>,
  #[serde(rename = "payload")]
  payload: Option<::models::MessagingAction>,
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl GetMessagingActionResponse {
  /// Describes a messaging action that can be taken for an order. Provides a JSON Hypertext Application Language (HAL) link to the JSON schema document that describes the expected input.
  pub fn new() -> GetMessagingActionResponse {
    GetMessagingActionResponse {
      _links: None,
      _embedded: None,
      payload: None,
      errors: None
    }
  }

  pub fn set__links(&mut self, _links: Value) {
    self._links = Some(_links);
  }

  pub fn with__links(mut self, _links: Value) -> GetMessagingActionResponse {
    self._links = Some(_links);
    self
  }

  pub fn _links(&self) -> Option<&Value> {
    self._links.as_ref()
  }

  pub fn reset__links(&mut self) {
    self._links = None;
  }

  pub fn set__embedded(&mut self, _embedded: Value) {
    self._embedded = Some(_embedded);
  }

  pub fn with__embedded(mut self, _embedded: Value) -> GetMessagingActionResponse {
    self._embedded = Some(_embedded);
    self
  }

  pub fn _embedded(&self) -> Option<&Value> {
    self._embedded.as_ref()
  }

  pub fn reset__embedded(&mut self) {
    self._embedded = None;
  }

  pub fn set_payload(&mut self, payload: ::models::MessagingAction) {
    self.payload = Some(payload);
  }

  pub fn with_payload(mut self, payload: ::models::MessagingAction) -> GetMessagingActionResponse {
    self.payload = Some(payload);
    self
  }

  pub fn payload(&self) -> Option<&::models::MessagingAction> {
    self.payload.as_ref()
  }

  pub fn reset_payload(&mut self) {
    self.payload = None;
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> GetMessagingActionResponse {
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



