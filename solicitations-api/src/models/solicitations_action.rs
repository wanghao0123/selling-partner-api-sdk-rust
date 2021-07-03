/* 
 * Selling Partner API for Solicitations
 *
 * With the Solicitations API you can build applications that send non-critical solicitations to buyers. You can get a list of solicitation types that are available for an order that you specify, then call an operation that sends a solicitation to the buyer for that order. Buyers cannot respond to solicitations sent by this API, and these solicitations do not appear in the Messaging section of Seller Central or in the recipient's Message Center. The Solicitations API returns responses that are formed according to the <a href=https://tools.ietf.org/html/draft-kelly-json-hal-08>JSON Hypertext Application Language</a> (HAL) standard.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SolicitationsAction : A simple object containing the name of the template.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SolicitationsAction {
  #[serde(rename = "name")]
  name: String
}

impl SolicitationsAction {
  /// A simple object containing the name of the template.
  pub fn new(name: String) -> SolicitationsAction {
    SolicitationsAction {
      name: name
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> SolicitationsAction {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


}



