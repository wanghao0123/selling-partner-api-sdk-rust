/* 
 * Selling Partner API for Solicitations
 *
 * With the Solicitations API you can build applications that send non-critical solicitations to buyers. You can get a list of solicitation types that are available for an order that you specify, then call an operation that sends a solicitation to the buyer for that order. Buyers cannot respond to solicitations sent by this API, and these solicitations do not appear in the Messaging section of Seller Central or in the recipient's Message Center. The Solicitations API returns responses that are formed according to the <a href=https://tools.ietf.org/html/draft-kelly-json-hal-08>JSON Hypertext Application Language</a> (HAL) standard.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateProductReviewAndSellerFeedbackSolicitationResponse : The response schema for the createProductReviewAndSellerFeedbackSolicitation operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductReviewAndSellerFeedbackSolicitationResponse {
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl CreateProductReviewAndSellerFeedbackSolicitationResponse {
  /// The response schema for the createProductReviewAndSellerFeedbackSolicitation operation.
  pub fn new() -> CreateProductReviewAndSellerFeedbackSolicitationResponse {
    CreateProductReviewAndSellerFeedbackSolicitationResponse {
      errors: None
    }
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> CreateProductReviewAndSellerFeedbackSolicitationResponse {
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



