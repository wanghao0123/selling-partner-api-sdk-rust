/* 
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// InvalidAsinList : A list of invalid ASIN values and the reasons they are invalid.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct InvalidAsinList {
}

impl InvalidAsinList {
  /// A list of invalid ASIN values and the reasons they are invalid.
  pub fn new() -> InvalidAsinList {
    InvalidAsinList {
    }
  }

}



