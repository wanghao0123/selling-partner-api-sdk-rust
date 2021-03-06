/* 
 * Selling Partner API for Direct Fulfillment Inventory Updates
 *
 * The Selling Partner API for Direct Fulfillment Inventory Updates provides programmatic access to a direct fulfillment vendor's inventory updates.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ErrorList : A list of error responses returned when a request is unsuccessful.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorList {
}

impl ErrorList {
  /// A list of error responses returned when a request is unsuccessful.
  pub fn new() -> ErrorList {
    ErrorList {
    }
  }

}



