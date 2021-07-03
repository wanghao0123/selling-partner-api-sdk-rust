/* 
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonTransportResult {
  #[serde(rename = "TransportResult")]
  transport_result: Option<::models::TransportResult>
}

impl CommonTransportResult {
  pub fn new() -> CommonTransportResult {
    CommonTransportResult {
      transport_result: None
    }
  }

  pub fn set_transport_result(&mut self, transport_result: ::models::TransportResult) {
    self.transport_result = Some(transport_result);
  }

  pub fn with_transport_result(mut self, transport_result: ::models::TransportResult) -> CommonTransportResult {
    self.transport_result = Some(transport_result);
    self
  }

  pub fn transport_result(&self) -> Option<&::models::TransportResult> {
    self.transport_result.as_ref()
  }

  pub fn reset_transport_result(&mut self) {
    self.transport_result = None;
  }

}



