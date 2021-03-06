/* 
 * Selling Partner API for Solicitations
 *
 * With the Solicitations API you can build applications that send non-critical solicitations to buyers. You can get a list of solicitation types that are available for an order that you specify, then call an operation that sends a solicitation to the buyer for that order. Buyers cannot respond to solicitations sent by this API, and these solicitations do not appear in the Messaging section of Seller Central or in the recipient's Message Center. The Solicitations API returns responses that are formed according to the <a href=https://tools.ietf.org/html/draft-kelly-json-hal-08>JSON Hypertext Application Language</a> (HAL) standard.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSchemaResponse {
  #[serde(rename = "_links")]
  _links: Option<Value>,
  #[serde(rename = "payload")]
  payload: Option<::models::Schema>,
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl GetSchemaResponse {
  pub fn new() -> GetSchemaResponse {
    GetSchemaResponse {
      _links: None,
      payload: None,
      errors: None
    }
  }

  pub fn set__links(&mut self, _links: Value) {
    self._links = Some(_links);
  }

  pub fn with__links(mut self, _links: Value) -> GetSchemaResponse {
    self._links = Some(_links);
    self
  }

  pub fn _links(&self) -> Option<&Value> {
    self._links.as_ref()
  }

  pub fn reset__links(&mut self) {
    self._links = None;
  }

  pub fn set_payload(&mut self, payload: ::models::Schema) {
    self.payload = Some(payload);
  }

  pub fn with_payload(mut self, payload: ::models::Schema) -> GetSchemaResponse {
    self.payload = Some(payload);
    self
  }

  pub fn payload(&self) -> Option<&::models::Schema> {
    self.payload.as_ref()
  }

  pub fn reset_payload(&mut self) {
    self.payload = None;
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> GetSchemaResponse {
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



