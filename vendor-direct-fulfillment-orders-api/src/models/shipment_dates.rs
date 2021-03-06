/* 
 * Selling Partner API for Direct Fulfillment Orders
 *
 * The Selling Partner API for Direct Fulfillment Orders provides programmatic access to a direct fulfillment vendor's order data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ShipmentDates : Shipment dates.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipmentDates {
  /// Time by which the vendor is required to ship the order.
  #[serde(rename = "requiredShipDate")]
  required_ship_date: String,
  /// Delivery date promised to the Amazon customer.
  #[serde(rename = "promisedDeliveryDate")]
  promised_delivery_date: Option<String>
}

impl ShipmentDates {
  /// Shipment dates.
  pub fn new(required_ship_date: String) -> ShipmentDates {
    ShipmentDates {
      required_ship_date: required_ship_date,
      promised_delivery_date: None
    }
  }

  pub fn set_required_ship_date(&mut self, required_ship_date: String) {
    self.required_ship_date = required_ship_date;
  }

  pub fn with_required_ship_date(mut self, required_ship_date: String) -> ShipmentDates {
    self.required_ship_date = required_ship_date;
    self
  }

  pub fn required_ship_date(&self) -> &String {
    &self.required_ship_date
  }


  pub fn set_promised_delivery_date(&mut self, promised_delivery_date: String) {
    self.promised_delivery_date = Some(promised_delivery_date);
  }

  pub fn with_promised_delivery_date(mut self, promised_delivery_date: String) -> ShipmentDates {
    self.promised_delivery_date = Some(promised_delivery_date);
    self
  }

  pub fn promised_delivery_date(&self) -> Option<&String> {
    self.promised_delivery_date.as_ref()
  }

  pub fn reset_promised_delivery_date(&mut self) {
    self.promised_delivery_date = None;
  }

}



