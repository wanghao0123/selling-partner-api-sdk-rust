/* 
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CustomTextForLabel : Custom text to print on the label.  Note: Custom text is only included on labels that are in ZPL format (ZPL203). FedEx does not support CustomTextForLabel.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomTextForLabel {
}

impl CustomTextForLabel {
  /// Custom text to print on the label.  Note: Custom text is only included on labels that are in ZPL format (ZPL203). FedEx does not support CustomTextForLabel.
  pub fn new() -> CustomTextForLabel {
    CustomTextForLabel {
    }
  }

}



