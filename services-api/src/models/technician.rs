/* 
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Technician : A technician who is assigned to perform the service job in part or in full.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Technician {
  /// The technician identifier.
  #[serde(rename = "technicianId")]
  technician_id: Option<String>,
  /// The name of the technician.
  #[serde(rename = "name")]
  name: Option<String>
}

impl Technician {
  /// A technician who is assigned to perform the service job in part or in full.
  pub fn new() -> Technician {
    Technician {
      technician_id: None,
      name: None
    }
  }

  pub fn set_technician_id(&mut self, technician_id: String) {
    self.technician_id = Some(technician_id);
  }

  pub fn with_technician_id(mut self, technician_id: String) -> Technician {
    self.technician_id = Some(technician_id);
    self
  }

  pub fn technician_id(&self) -> Option<&String> {
    self.technician_id.as_ref()
  }

  pub fn reset_technician_id(&mut self) {
    self.technician_id = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> Technician {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

}



