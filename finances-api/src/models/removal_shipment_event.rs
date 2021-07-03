/* 
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RemovalShipmentEvent : A removal shipment event for a removal order.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovalShipmentEvent {
  /// The date and time when the financial event was posted.
  #[serde(rename = "PostedDate")]
  posted_date: Option<::models::Date>,
  /// The identifier for the removal shipment order.
  #[serde(rename = "OrderId")]
  order_id: Option<String>,
  /// The type of removal order.  Possible values:  * WHOLESALE_LIQUIDATION
  #[serde(rename = "TransactionType")]
  transaction_type: Option<String>,
  /// A list of removal shipment items.
  #[serde(rename = "RemovalShipmentItemList")]
  removal_shipment_item_list: Option<::models::RemovalShipmentItemList>
}

impl RemovalShipmentEvent {
  /// A removal shipment event for a removal order.
  pub fn new() -> RemovalShipmentEvent {
    RemovalShipmentEvent {
      posted_date: None,
      order_id: None,
      transaction_type: None,
      removal_shipment_item_list: None
    }
  }

  pub fn set_posted_date(&mut self, posted_date: ::models::Date) {
    self.posted_date = Some(posted_date);
  }

  pub fn with_posted_date(mut self, posted_date: ::models::Date) -> RemovalShipmentEvent {
    self.posted_date = Some(posted_date);
    self
  }

  pub fn posted_date(&self) -> Option<&::models::Date> {
    self.posted_date.as_ref()
  }

  pub fn reset_posted_date(&mut self) {
    self.posted_date = None;
  }

  pub fn set_order_id(&mut self, order_id: String) {
    self.order_id = Some(order_id);
  }

  pub fn with_order_id(mut self, order_id: String) -> RemovalShipmentEvent {
    self.order_id = Some(order_id);
    self
  }

  pub fn order_id(&self) -> Option<&String> {
    self.order_id.as_ref()
  }

  pub fn reset_order_id(&mut self) {
    self.order_id = None;
  }

  pub fn set_transaction_type(&mut self, transaction_type: String) {
    self.transaction_type = Some(transaction_type);
  }

  pub fn with_transaction_type(mut self, transaction_type: String) -> RemovalShipmentEvent {
    self.transaction_type = Some(transaction_type);
    self
  }

  pub fn transaction_type(&self) -> Option<&String> {
    self.transaction_type.as_ref()
  }

  pub fn reset_transaction_type(&mut self) {
    self.transaction_type = None;
  }

  pub fn set_removal_shipment_item_list(&mut self, removal_shipment_item_list: ::models::RemovalShipmentItemList) {
    self.removal_shipment_item_list = Some(removal_shipment_item_list);
  }

  pub fn with_removal_shipment_item_list(mut self, removal_shipment_item_list: ::models::RemovalShipmentItemList) -> RemovalShipmentEvent {
    self.removal_shipment_item_list = Some(removal_shipment_item_list);
    self
  }

  pub fn removal_shipment_item_list(&self) -> Option<&::models::RemovalShipmentItemList> {
    self.removal_shipment_item_list.as_ref()
  }

  pub fn reset_removal_shipment_item_list(&mut self) {
    self.removal_shipment_item_list = None;
  }

}



