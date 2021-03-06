/* 
 * Selling Partner API for Retail Procurement Shipments
 *
 * The Selling Partner API for Retail Procurement Shipments provides programmatic access to retail shipping data for vendors.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportDetails {
  /// This is used for import purchase orders only. If the recipient requests, this field will contain the shipment method of payment.
  #[serde(rename = "methodOfPayment")]
  method_of_payment: Option<String>,
  /// The container's seal number.
  #[serde(rename = "sealNumber")]
  seal_number: Option<String>,
  /// The route and stop details for this shipment.
  #[serde(rename = "route")]
  route: Option<::models::Route>,
  /// Types and numbers of container(s) for import purchase orders. Can be a comma-separated list if shipment has multiple containers.
  #[serde(rename = "importContainers")]
  import_containers: Option<String>,
  /// Billable weight of the direct imports shipment.
  #[serde(rename = "billableWeight")]
  billable_weight: Option<::models::Weight>,
  /// Date on which the shipment is expected to be shipped. This value should not be in the past and not more than 60 days out in the future.
  #[serde(rename = "estimatedShipByDate")]
  estimated_ship_by_date: Option<String>
}

impl ImportDetails {
  pub fn new() -> ImportDetails {
    ImportDetails {
      method_of_payment: None,
      seal_number: None,
      route: None,
      import_containers: None,
      billable_weight: None,
      estimated_ship_by_date: None
    }
  }

  pub fn set_method_of_payment(&mut self, method_of_payment: String) {
    self.method_of_payment = Some(method_of_payment);
  }

  pub fn with_method_of_payment(mut self, method_of_payment: String) -> ImportDetails {
    self.method_of_payment = Some(method_of_payment);
    self
  }

  pub fn method_of_payment(&self) -> Option<&String> {
    self.method_of_payment.as_ref()
  }

  pub fn reset_method_of_payment(&mut self) {
    self.method_of_payment = None;
  }

  pub fn set_seal_number(&mut self, seal_number: String) {
    self.seal_number = Some(seal_number);
  }

  pub fn with_seal_number(mut self, seal_number: String) -> ImportDetails {
    self.seal_number = Some(seal_number);
    self
  }

  pub fn seal_number(&self) -> Option<&String> {
    self.seal_number.as_ref()
  }

  pub fn reset_seal_number(&mut self) {
    self.seal_number = None;
  }

  pub fn set_route(&mut self, route: ::models::Route) {
    self.route = Some(route);
  }

  pub fn with_route(mut self, route: ::models::Route) -> ImportDetails {
    self.route = Some(route);
    self
  }

  pub fn route(&self) -> Option<&::models::Route> {
    self.route.as_ref()
  }

  pub fn reset_route(&mut self) {
    self.route = None;
  }

  pub fn set_import_containers(&mut self, import_containers: String) {
    self.import_containers = Some(import_containers);
  }

  pub fn with_import_containers(mut self, import_containers: String) -> ImportDetails {
    self.import_containers = Some(import_containers);
    self
  }

  pub fn import_containers(&self) -> Option<&String> {
    self.import_containers.as_ref()
  }

  pub fn reset_import_containers(&mut self) {
    self.import_containers = None;
  }

  pub fn set_billable_weight(&mut self, billable_weight: ::models::Weight) {
    self.billable_weight = Some(billable_weight);
  }

  pub fn with_billable_weight(mut self, billable_weight: ::models::Weight) -> ImportDetails {
    self.billable_weight = Some(billable_weight);
    self
  }

  pub fn billable_weight(&self) -> Option<&::models::Weight> {
    self.billable_weight.as_ref()
  }

  pub fn reset_billable_weight(&mut self) {
    self.billable_weight = None;
  }

  pub fn set_estimated_ship_by_date(&mut self, estimated_ship_by_date: String) {
    self.estimated_ship_by_date = Some(estimated_ship_by_date);
  }

  pub fn with_estimated_ship_by_date(mut self, estimated_ship_by_date: String) -> ImportDetails {
    self.estimated_ship_by_date = Some(estimated_ship_by_date);
    self
  }

  pub fn estimated_ship_by_date(&self) -> Option<&String> {
    self.estimated_ship_by_date.as_ref()
  }

  pub fn reset_estimated_ship_by_date(&mut self) {
    self.estimated_ship_by_date = None;
  }

}



