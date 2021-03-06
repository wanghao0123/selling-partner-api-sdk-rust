/* 
 * Selling Partner API for Direct Fulfillment Shipping
 *
 * The Selling Partner API for Direct Fulfillment Shipping provides programmatic access to a direct fulfillment vendor's shipping data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
  /// A generated string used to pass information to your next request. If NextToken is returned, pass the value of NextToken to the next request. If NextToken is not returned, there are no more order items to return.
  #[serde(rename = "nextToken")]
  next_token: Option<String>
}

impl Pagination {
  pub fn new() -> Pagination {
    Pagination {
      next_token: None
    }
  }

  pub fn set_next_token(&mut self, next_token: String) {
    self.next_token = Some(next_token);
  }

  pub fn with_next_token(mut self, next_token: String) -> Pagination {
    self.next_token = Some(next_token);
    self
  }

  pub fn next_token(&self) -> Option<&String> {
    self.next_token.as_ref()
  }

  pub fn reset_next_token(&mut self) {
    self.next_token = None;
  }

}



