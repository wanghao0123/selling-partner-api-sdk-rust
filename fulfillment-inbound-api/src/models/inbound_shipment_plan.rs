/* 
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// InboundShipmentPlan : Inbound shipment information used to create an inbound shipment. Returned by the createInboundShipmentPlan operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct InboundShipmentPlan {
  /// A shipment identifier originally returned by the createInboundShipmentPlan operation.
  #[serde(rename = "ShipmentId")]
  shipment_id: String,
  /// An Amazon fulfillment center identifier created by Amazon.
  #[serde(rename = "DestinationFulfillmentCenterId")]
  destination_fulfillment_center_id: String,
  /// The address of the Amazon fulfillment center to which to ship the items.
  #[serde(rename = "ShipToAddress")]
  ship_to_address: ::models::Address,
  #[serde(rename = "LabelPrepType")]
  label_prep_type: ::models::LabelPrepType,
  /// SKU and quantity information for the items in the shipment.
  #[serde(rename = "Items")]
  items: ::models::InboundShipmentPlanItemList,
  #[serde(rename = "EstimatedBoxContentsFee")]
  estimated_box_contents_fee: Option<::models::BoxContentsFeeDetails>
}

impl InboundShipmentPlan {
  /// Inbound shipment information used to create an inbound shipment. Returned by the createInboundShipmentPlan operation.
  pub fn new(shipment_id: String, destination_fulfillment_center_id: String, ship_to_address: ::models::Address, label_prep_type: ::models::LabelPrepType, items: ::models::InboundShipmentPlanItemList) -> InboundShipmentPlan {
    InboundShipmentPlan {
      shipment_id: shipment_id,
      destination_fulfillment_center_id: destination_fulfillment_center_id,
      ship_to_address: ship_to_address,
      label_prep_type: label_prep_type,
      items: items,
      estimated_box_contents_fee: None
    }
  }

  pub fn set_shipment_id(&mut self, shipment_id: String) {
    self.shipment_id = shipment_id;
  }

  pub fn with_shipment_id(mut self, shipment_id: String) -> InboundShipmentPlan {
    self.shipment_id = shipment_id;
    self
  }

  pub fn shipment_id(&self) -> &String {
    &self.shipment_id
  }


  pub fn set_destination_fulfillment_center_id(&mut self, destination_fulfillment_center_id: String) {
    self.destination_fulfillment_center_id = destination_fulfillment_center_id;
  }

  pub fn with_destination_fulfillment_center_id(mut self, destination_fulfillment_center_id: String) -> InboundShipmentPlan {
    self.destination_fulfillment_center_id = destination_fulfillment_center_id;
    self
  }

  pub fn destination_fulfillment_center_id(&self) -> &String {
    &self.destination_fulfillment_center_id
  }


  pub fn set_ship_to_address(&mut self, ship_to_address: ::models::Address) {
    self.ship_to_address = ship_to_address;
  }

  pub fn with_ship_to_address(mut self, ship_to_address: ::models::Address) -> InboundShipmentPlan {
    self.ship_to_address = ship_to_address;
    self
  }

  pub fn ship_to_address(&self) -> &::models::Address {
    &self.ship_to_address
  }


  pub fn set_label_prep_type(&mut self, label_prep_type: ::models::LabelPrepType) {
    self.label_prep_type = label_prep_type;
  }

  pub fn with_label_prep_type(mut self, label_prep_type: ::models::LabelPrepType) -> InboundShipmentPlan {
    self.label_prep_type = label_prep_type;
    self
  }

  pub fn label_prep_type(&self) -> &::models::LabelPrepType {
    &self.label_prep_type
  }


  pub fn set_items(&mut self, items: ::models::InboundShipmentPlanItemList) {
    self.items = items;
  }

  pub fn with_items(mut self, items: ::models::InboundShipmentPlanItemList) -> InboundShipmentPlan {
    self.items = items;
    self
  }

  pub fn items(&self) -> &::models::InboundShipmentPlanItemList {
    &self.items
  }


  pub fn set_estimated_box_contents_fee(&mut self, estimated_box_contents_fee: ::models::BoxContentsFeeDetails) {
    self.estimated_box_contents_fee = Some(estimated_box_contents_fee);
  }

  pub fn with_estimated_box_contents_fee(mut self, estimated_box_contents_fee: ::models::BoxContentsFeeDetails) -> InboundShipmentPlan {
    self.estimated_box_contents_fee = Some(estimated_box_contents_fee);
    self
  }

  pub fn estimated_box_contents_fee(&self) -> Option<&::models::BoxContentsFeeDetails> {
    self.estimated_box_contents_fee.as_ref()
  }

  pub fn reset_estimated_box_contents_fee(&mut self) {
    self.estimated_box_contents_fee = None;
  }

}



