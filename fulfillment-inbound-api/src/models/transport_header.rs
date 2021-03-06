/* 
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// TransportHeader : The shipping identifier, information about whether the shipment is by an Amazon-partnered carrier, and information about whether the shipment is Small Parcel or Less Than Truckload/Full Truckload (LTL/FTL).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportHeader {
  /// The Amazon seller identifier.
  #[serde(rename = "SellerId")]
  seller_id: String,
  /// A shipment identifier originally returned by the createInboundShipmentPlan operation.
  #[serde(rename = "ShipmentId")]
  shipment_id: String,
  /// Indicates whether a putTransportDetails request is for a partnered carrier.  Possible values:  * true – Request is for an Amazon-partnered carrier.  * false – Request is for a non-Amazon-partnered carrier.
  #[serde(rename = "IsPartnered")]
  is_partnered: bool,
  #[serde(rename = "ShipmentType")]
  shipment_type: ::models::ShipmentType
}

impl TransportHeader {
  /// The shipping identifier, information about whether the shipment is by an Amazon-partnered carrier, and information about whether the shipment is Small Parcel or Less Than Truckload/Full Truckload (LTL/FTL).
  pub fn new(seller_id: String, shipment_id: String, is_partnered: bool, shipment_type: ::models::ShipmentType) -> TransportHeader {
    TransportHeader {
      seller_id: seller_id,
      shipment_id: shipment_id,
      is_partnered: is_partnered,
      shipment_type: shipment_type
    }
  }

  pub fn set_seller_id(&mut self, seller_id: String) {
    self.seller_id = seller_id;
  }

  pub fn with_seller_id(mut self, seller_id: String) -> TransportHeader {
    self.seller_id = seller_id;
    self
  }

  pub fn seller_id(&self) -> &String {
    &self.seller_id
  }


  pub fn set_shipment_id(&mut self, shipment_id: String) {
    self.shipment_id = shipment_id;
  }

  pub fn with_shipment_id(mut self, shipment_id: String) -> TransportHeader {
    self.shipment_id = shipment_id;
    self
  }

  pub fn shipment_id(&self) -> &String {
    &self.shipment_id
  }


  pub fn set_is_partnered(&mut self, is_partnered: bool) {
    self.is_partnered = is_partnered;
  }

  pub fn with_is_partnered(mut self, is_partnered: bool) -> TransportHeader {
    self.is_partnered = is_partnered;
    self
  }

  pub fn is_partnered(&self) -> &bool {
    &self.is_partnered
  }


  pub fn set_shipment_type(&mut self, shipment_type: ::models::ShipmentType) {
    self.shipment_type = shipment_type;
  }

  pub fn with_shipment_type(mut self, shipment_type: ::models::ShipmentType) -> TransportHeader {
    self.shipment_type = shipment_type;
    self
  }

  pub fn shipment_type(&self) -> &::models::ShipmentType {
    &self.shipment_type
  }


}



