/* 
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ShipmentEvent : A shipment, refund, guarantee claim, or chargeback.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipmentEvent {
  /// An Amazon-defined identifier for an order.
  #[serde(rename = "AmazonOrderId")]
  amazon_order_id: Option<String>,
  /// A seller-defined identifier for an order.
  #[serde(rename = "SellerOrderId")]
  seller_order_id: Option<String>,
  /// The name of the marketplace where the event occurred.
  #[serde(rename = "MarketplaceName")]
  marketplace_name: Option<String>,
  /// A list of order-level charges. These charges are applicable to Multi-Channel Fulfillment COD orders.
  #[serde(rename = "OrderChargeList")]
  order_charge_list: Option<::models::ChargeComponentList>,
  /// A list of order-level charge adjustments. These adjustments are applicable to Multi-Channel Fulfillment COD orders.
  #[serde(rename = "OrderChargeAdjustmentList")]
  order_charge_adjustment_list: Option<::models::ChargeComponentList>,
  /// A list of shipment-level fees.
  #[serde(rename = "ShipmentFeeList")]
  shipment_fee_list: Option<::models::FeeComponentList>,
  /// A list of shipment-level fee adjustments.
  #[serde(rename = "ShipmentFeeAdjustmentList")]
  shipment_fee_adjustment_list: Option<::models::FeeComponentList>,
  /// A list of order-level fees. These charges are applicable to Multi-Channel Fulfillment orders.
  #[serde(rename = "OrderFeeList")]
  order_fee_list: Option<::models::FeeComponentList>,
  /// A list of order-level fee adjustments. These adjustments are applicable to Multi-Channel Fulfillment orders.
  #[serde(rename = "OrderFeeAdjustmentList")]
  order_fee_adjustment_list: Option<::models::FeeComponentList>,
  /// A list of transactions where buyers pay Amazon through one of the credit cards offered by Amazon or where buyers pay a seller directly through COD.
  #[serde(rename = "DirectPaymentList")]
  direct_payment_list: Option<::models::DirectPaymentList>,
  /// The date and time when the financial event was posted.
  #[serde(rename = "PostedDate")]
  posted_date: Option<::models::Date>,
  #[serde(rename = "ShipmentItemList")]
  shipment_item_list: Option<::models::ShipmentItemList>,
  /// A list of shipment item adjustments.
  #[serde(rename = "ShipmentItemAdjustmentList")]
  shipment_item_adjustment_list: Option<::models::ShipmentItemList>
}

impl ShipmentEvent {
  /// A shipment, refund, guarantee claim, or chargeback.
  pub fn new() -> ShipmentEvent {
    ShipmentEvent {
      amazon_order_id: None,
      seller_order_id: None,
      marketplace_name: None,
      order_charge_list: None,
      order_charge_adjustment_list: None,
      shipment_fee_list: None,
      shipment_fee_adjustment_list: None,
      order_fee_list: None,
      order_fee_adjustment_list: None,
      direct_payment_list: None,
      posted_date: None,
      shipment_item_list: None,
      shipment_item_adjustment_list: None
    }
  }

  pub fn set_amazon_order_id(&mut self, amazon_order_id: String) {
    self.amazon_order_id = Some(amazon_order_id);
  }

  pub fn with_amazon_order_id(mut self, amazon_order_id: String) -> ShipmentEvent {
    self.amazon_order_id = Some(amazon_order_id);
    self
  }

  pub fn amazon_order_id(&self) -> Option<&String> {
    self.amazon_order_id.as_ref()
  }

  pub fn reset_amazon_order_id(&mut self) {
    self.amazon_order_id = None;
  }

  pub fn set_seller_order_id(&mut self, seller_order_id: String) {
    self.seller_order_id = Some(seller_order_id);
  }

  pub fn with_seller_order_id(mut self, seller_order_id: String) -> ShipmentEvent {
    self.seller_order_id = Some(seller_order_id);
    self
  }

  pub fn seller_order_id(&self) -> Option<&String> {
    self.seller_order_id.as_ref()
  }

  pub fn reset_seller_order_id(&mut self) {
    self.seller_order_id = None;
  }

  pub fn set_marketplace_name(&mut self, marketplace_name: String) {
    self.marketplace_name = Some(marketplace_name);
  }

  pub fn with_marketplace_name(mut self, marketplace_name: String) -> ShipmentEvent {
    self.marketplace_name = Some(marketplace_name);
    self
  }

  pub fn marketplace_name(&self) -> Option<&String> {
    self.marketplace_name.as_ref()
  }

  pub fn reset_marketplace_name(&mut self) {
    self.marketplace_name = None;
  }

  pub fn set_order_charge_list(&mut self, order_charge_list: ::models::ChargeComponentList) {
    self.order_charge_list = Some(order_charge_list);
  }

  pub fn with_order_charge_list(mut self, order_charge_list: ::models::ChargeComponentList) -> ShipmentEvent {
    self.order_charge_list = Some(order_charge_list);
    self
  }

  pub fn order_charge_list(&self) -> Option<&::models::ChargeComponentList> {
    self.order_charge_list.as_ref()
  }

  pub fn reset_order_charge_list(&mut self) {
    self.order_charge_list = None;
  }

  pub fn set_order_charge_adjustment_list(&mut self, order_charge_adjustment_list: ::models::ChargeComponentList) {
    self.order_charge_adjustment_list = Some(order_charge_adjustment_list);
  }

  pub fn with_order_charge_adjustment_list(mut self, order_charge_adjustment_list: ::models::ChargeComponentList) -> ShipmentEvent {
    self.order_charge_adjustment_list = Some(order_charge_adjustment_list);
    self
  }

  pub fn order_charge_adjustment_list(&self) -> Option<&::models::ChargeComponentList> {
    self.order_charge_adjustment_list.as_ref()
  }

  pub fn reset_order_charge_adjustment_list(&mut self) {
    self.order_charge_adjustment_list = None;
  }

  pub fn set_shipment_fee_list(&mut self, shipment_fee_list: ::models::FeeComponentList) {
    self.shipment_fee_list = Some(shipment_fee_list);
  }

  pub fn with_shipment_fee_list(mut self, shipment_fee_list: ::models::FeeComponentList) -> ShipmentEvent {
    self.shipment_fee_list = Some(shipment_fee_list);
    self
  }

  pub fn shipment_fee_list(&self) -> Option<&::models::FeeComponentList> {
    self.shipment_fee_list.as_ref()
  }

  pub fn reset_shipment_fee_list(&mut self) {
    self.shipment_fee_list = None;
  }

  pub fn set_shipment_fee_adjustment_list(&mut self, shipment_fee_adjustment_list: ::models::FeeComponentList) {
    self.shipment_fee_adjustment_list = Some(shipment_fee_adjustment_list);
  }

  pub fn with_shipment_fee_adjustment_list(mut self, shipment_fee_adjustment_list: ::models::FeeComponentList) -> ShipmentEvent {
    self.shipment_fee_adjustment_list = Some(shipment_fee_adjustment_list);
    self
  }

  pub fn shipment_fee_adjustment_list(&self) -> Option<&::models::FeeComponentList> {
    self.shipment_fee_adjustment_list.as_ref()
  }

  pub fn reset_shipment_fee_adjustment_list(&mut self) {
    self.shipment_fee_adjustment_list = None;
  }

  pub fn set_order_fee_list(&mut self, order_fee_list: ::models::FeeComponentList) {
    self.order_fee_list = Some(order_fee_list);
  }

  pub fn with_order_fee_list(mut self, order_fee_list: ::models::FeeComponentList) -> ShipmentEvent {
    self.order_fee_list = Some(order_fee_list);
    self
  }

  pub fn order_fee_list(&self) -> Option<&::models::FeeComponentList> {
    self.order_fee_list.as_ref()
  }

  pub fn reset_order_fee_list(&mut self) {
    self.order_fee_list = None;
  }

  pub fn set_order_fee_adjustment_list(&mut self, order_fee_adjustment_list: ::models::FeeComponentList) {
    self.order_fee_adjustment_list = Some(order_fee_adjustment_list);
  }

  pub fn with_order_fee_adjustment_list(mut self, order_fee_adjustment_list: ::models::FeeComponentList) -> ShipmentEvent {
    self.order_fee_adjustment_list = Some(order_fee_adjustment_list);
    self
  }

  pub fn order_fee_adjustment_list(&self) -> Option<&::models::FeeComponentList> {
    self.order_fee_adjustment_list.as_ref()
  }

  pub fn reset_order_fee_adjustment_list(&mut self) {
    self.order_fee_adjustment_list = None;
  }

  pub fn set_direct_payment_list(&mut self, direct_payment_list: ::models::DirectPaymentList) {
    self.direct_payment_list = Some(direct_payment_list);
  }

  pub fn with_direct_payment_list(mut self, direct_payment_list: ::models::DirectPaymentList) -> ShipmentEvent {
    self.direct_payment_list = Some(direct_payment_list);
    self
  }

  pub fn direct_payment_list(&self) -> Option<&::models::DirectPaymentList> {
    self.direct_payment_list.as_ref()
  }

  pub fn reset_direct_payment_list(&mut self) {
    self.direct_payment_list = None;
  }

  pub fn set_posted_date(&mut self, posted_date: ::models::Date) {
    self.posted_date = Some(posted_date);
  }

  pub fn with_posted_date(mut self, posted_date: ::models::Date) -> ShipmentEvent {
    self.posted_date = Some(posted_date);
    self
  }

  pub fn posted_date(&self) -> Option<&::models::Date> {
    self.posted_date.as_ref()
  }

  pub fn reset_posted_date(&mut self) {
    self.posted_date = None;
  }

  pub fn set_shipment_item_list(&mut self, shipment_item_list: ::models::ShipmentItemList) {
    self.shipment_item_list = Some(shipment_item_list);
  }

  pub fn with_shipment_item_list(mut self, shipment_item_list: ::models::ShipmentItemList) -> ShipmentEvent {
    self.shipment_item_list = Some(shipment_item_list);
    self
  }

  pub fn shipment_item_list(&self) -> Option<&::models::ShipmentItemList> {
    self.shipment_item_list.as_ref()
  }

  pub fn reset_shipment_item_list(&mut self) {
    self.shipment_item_list = None;
  }

  pub fn set_shipment_item_adjustment_list(&mut self, shipment_item_adjustment_list: ::models::ShipmentItemList) {
    self.shipment_item_adjustment_list = Some(shipment_item_adjustment_list);
  }

  pub fn with_shipment_item_adjustment_list(mut self, shipment_item_adjustment_list: ::models::ShipmentItemList) -> ShipmentEvent {
    self.shipment_item_adjustment_list = Some(shipment_item_adjustment_list);
    self
  }

  pub fn shipment_item_adjustment_list(&self) -> Option<&::models::ShipmentItemList> {
    self.shipment_item_adjustment_list.as_ref()
  }

  pub fn reset_shipment_item_adjustment_list(&mut self) {
    self.shipment_item_adjustment_list = None;
  }

}



