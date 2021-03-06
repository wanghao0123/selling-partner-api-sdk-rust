/* 
 * Selling Partner API for Retail Procurement Shipments
 *
 * The Selling Partner API for Retail Procurement Shipments provides programmatic access to retail shipping data for vendors.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SubmitShipmentConfirmationsRequest : The request schema for the SubmitShipmentConfirmations operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitShipmentConfirmationsRequest {
  #[serde(rename = "shipmentConfirmations")]
  shipment_confirmations: Option<Vec<::models::ShipmentConfirmation>>
}

impl SubmitShipmentConfirmationsRequest {
  /// The request schema for the SubmitShipmentConfirmations operation.
  pub fn new() -> SubmitShipmentConfirmationsRequest {
    SubmitShipmentConfirmationsRequest {
      shipment_confirmations: None
    }
  }

  pub fn set_shipment_confirmations(&mut self, shipment_confirmations: Vec<::models::ShipmentConfirmation>) {
    self.shipment_confirmations = Some(shipment_confirmations);
  }

  pub fn with_shipment_confirmations(mut self, shipment_confirmations: Vec<::models::ShipmentConfirmation>) -> SubmitShipmentConfirmationsRequest {
    self.shipment_confirmations = Some(shipment_confirmations);
    self
  }

  pub fn shipment_confirmations(&self) -> Option<&Vec<::models::ShipmentConfirmation>> {
    self.shipment_confirmations.as_ref()
  }

  pub fn reset_shipment_confirmations(&mut self) {
    self.shipment_confirmations = None;
  }

}



