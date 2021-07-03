/* 
 * Selling Partner API for FBA Small And Light
 *
 * The Selling Partner API for FBA Small and Light lets you help sellers manage their listings in the Small and Light program. The program reduces the cost of fulfilling orders for small and lightweight FBA inventory. You can enroll or remove items from the program and check item eligibility and enrollment status. You can also preview the estimated program fees charged to a seller for items sold while enrolled in the program.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallAndLightFeePreviews {
  /// A list of fee estimates for the requested items. The order of the fee estimates will follow the same order as the items in the request, with duplicates removed.
  #[serde(rename = "data")]
  data: Option<Vec<::models::FeePreview>>
}

impl SmallAndLightFeePreviews {
  pub fn new() -> SmallAndLightFeePreviews {
    SmallAndLightFeePreviews {
      data: None
    }
  }

  pub fn set_data(&mut self, data: Vec<::models::FeePreview>) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: Vec<::models::FeePreview>) -> SmallAndLightFeePreviews {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&Vec<::models::FeePreview>> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

}



