/* 
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AdjustmentEventList : A list of adjustment event information for the seller's account.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdjustmentEventList {
}

impl AdjustmentEventList {
  /// A list of adjustment event information for the seller's account.
  pub fn new() -> AdjustmentEventList {
    AdjustmentEventList {
    }
  }

}



