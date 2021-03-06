/* 
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// TransportContent : Inbound shipment information, including carrier details, shipment status, and the workflow status for a request for shipment with an Amazon-partnered carrier.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportContent {
  #[serde(rename = "TransportHeader")]
  transport_header: ::models::TransportHeader,
  #[serde(rename = "TransportDetails")]
  transport_details: ::models::TransportDetailOutput,
  #[serde(rename = "TransportResult")]
  transport_result: ::models::TransportResult
}

impl TransportContent {
  /// Inbound shipment information, including carrier details, shipment status, and the workflow status for a request for shipment with an Amazon-partnered carrier.
  pub fn new(transport_header: ::models::TransportHeader, transport_details: ::models::TransportDetailOutput, transport_result: ::models::TransportResult) -> TransportContent {
    TransportContent {
      transport_header: transport_header,
      transport_details: transport_details,
      transport_result: transport_result
    }
  }

  pub fn set_transport_header(&mut self, transport_header: ::models::TransportHeader) {
    self.transport_header = transport_header;
  }

  pub fn with_transport_header(mut self, transport_header: ::models::TransportHeader) -> TransportContent {
    self.transport_header = transport_header;
    self
  }

  pub fn transport_header(&self) -> &::models::TransportHeader {
    &self.transport_header
  }


  pub fn set_transport_details(&mut self, transport_details: ::models::TransportDetailOutput) {
    self.transport_details = transport_details;
  }

  pub fn with_transport_details(mut self, transport_details: ::models::TransportDetailOutput) -> TransportContent {
    self.transport_details = transport_details;
    self
  }

  pub fn transport_details(&self) -> &::models::TransportDetailOutput {
    &self.transport_details
  }


  pub fn set_transport_result(&mut self, transport_result: ::models::TransportResult) {
    self.transport_result = transport_result;
  }

  pub fn with_transport_result(mut self, transport_result: ::models::TransportResult) -> TransportContent {
    self.transport_result = transport_result;
    self
  }

  pub fn transport_result(&self) -> &::models::TransportResult {
    &self.transport_result
  }


}



