/* 
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateShipmentRequest : Request schema.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateShipmentRequest {
  /// Shipment information required for creating a shipment.
  #[serde(rename = "ShipmentRequestDetails")]
  shipment_request_details: ::models::ShipmentRequestDetails,
  #[serde(rename = "ShippingServiceId")]
  shipping_service_id: ::models::ShippingServiceIdentifier,
  /// Identifies a shipping service order made by a carrier.
  #[serde(rename = "ShippingServiceOfferId")]
  shipping_service_offer_id: Option<String>,
  /// Hazardous materials options for a package. Consult the terms and conditions for each carrier for more information about hazardous materials.
  #[serde(rename = "HazmatType")]
  hazmat_type: Option<::models::HazmatType>,
  #[serde(rename = "LabelFormatOption")]
  label_format_option: Option<::models::LabelFormatOptionRequest>,
  /// A list of additional seller inputs required to ship this shipment.
  #[serde(rename = "ShipmentLevelSellerInputsList")]
  shipment_level_seller_inputs_list: Option<::models::AdditionalSellerInputsList>
}

impl CreateShipmentRequest {
  /// Request schema.
  pub fn new(shipment_request_details: ::models::ShipmentRequestDetails, shipping_service_id: ::models::ShippingServiceIdentifier) -> CreateShipmentRequest {
    CreateShipmentRequest {
      shipment_request_details: shipment_request_details,
      shipping_service_id: shipping_service_id,
      shipping_service_offer_id: None,
      hazmat_type: None,
      label_format_option: None,
      shipment_level_seller_inputs_list: None
    }
  }

  pub fn set_shipment_request_details(&mut self, shipment_request_details: ::models::ShipmentRequestDetails) {
    self.shipment_request_details = shipment_request_details;
  }

  pub fn with_shipment_request_details(mut self, shipment_request_details: ::models::ShipmentRequestDetails) -> CreateShipmentRequest {
    self.shipment_request_details = shipment_request_details;
    self
  }

  pub fn shipment_request_details(&self) -> &::models::ShipmentRequestDetails {
    &self.shipment_request_details
  }


  pub fn set_shipping_service_id(&mut self, shipping_service_id: ::models::ShippingServiceIdentifier) {
    self.shipping_service_id = shipping_service_id;
  }

  pub fn with_shipping_service_id(mut self, shipping_service_id: ::models::ShippingServiceIdentifier) -> CreateShipmentRequest {
    self.shipping_service_id = shipping_service_id;
    self
  }

  pub fn shipping_service_id(&self) -> &::models::ShippingServiceIdentifier {
    &self.shipping_service_id
  }


  pub fn set_shipping_service_offer_id(&mut self, shipping_service_offer_id: String) {
    self.shipping_service_offer_id = Some(shipping_service_offer_id);
  }

  pub fn with_shipping_service_offer_id(mut self, shipping_service_offer_id: String) -> CreateShipmentRequest {
    self.shipping_service_offer_id = Some(shipping_service_offer_id);
    self
  }

  pub fn shipping_service_offer_id(&self) -> Option<&String> {
    self.shipping_service_offer_id.as_ref()
  }

  pub fn reset_shipping_service_offer_id(&mut self) {
    self.shipping_service_offer_id = None;
  }

  pub fn set_hazmat_type(&mut self, hazmat_type: ::models::HazmatType) {
    self.hazmat_type = Some(hazmat_type);
  }

  pub fn with_hazmat_type(mut self, hazmat_type: ::models::HazmatType) -> CreateShipmentRequest {
    self.hazmat_type = Some(hazmat_type);
    self
  }

  pub fn hazmat_type(&self) -> Option<&::models::HazmatType> {
    self.hazmat_type.as_ref()
  }

  pub fn reset_hazmat_type(&mut self) {
    self.hazmat_type = None;
  }

  pub fn set_label_format_option(&mut self, label_format_option: ::models::LabelFormatOptionRequest) {
    self.label_format_option = Some(label_format_option);
  }

  pub fn with_label_format_option(mut self, label_format_option: ::models::LabelFormatOptionRequest) -> CreateShipmentRequest {
    self.label_format_option = Some(label_format_option);
    self
  }

  pub fn label_format_option(&self) -> Option<&::models::LabelFormatOptionRequest> {
    self.label_format_option.as_ref()
  }

  pub fn reset_label_format_option(&mut self) {
    self.label_format_option = None;
  }

  pub fn set_shipment_level_seller_inputs_list(&mut self, shipment_level_seller_inputs_list: ::models::AdditionalSellerInputsList) {
    self.shipment_level_seller_inputs_list = Some(shipment_level_seller_inputs_list);
  }

  pub fn with_shipment_level_seller_inputs_list(mut self, shipment_level_seller_inputs_list: ::models::AdditionalSellerInputsList) -> CreateShipmentRequest {
    self.shipment_level_seller_inputs_list = Some(shipment_level_seller_inputs_list);
    self
  }

  pub fn shipment_level_seller_inputs_list(&self) -> Option<&::models::AdditionalSellerInputsList> {
    self.shipment_level_seller_inputs_list.as_ref()
  }

  pub fn reset_shipment_level_seller_inputs_list(&mut self) {
    self.shipment_level_seller_inputs_list = None;
  }

}



