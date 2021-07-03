/* 
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * OpenAPI spec version: 2020-07-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NotificationEmailList : A list of email addresses that the seller provides that are used by Amazon to send ship-complete notifications to recipients on behalf of the seller.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationEmailList {
}

impl NotificationEmailList {
  /// A list of email addresses that the seller provides that are used by Amazon to send ship-complete notifications to recipients on behalf of the seller.
  pub fn new() -> NotificationEmailList {
    NotificationEmailList {
    }
  }

}



