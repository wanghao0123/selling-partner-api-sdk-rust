/* 
 * Selling Partner API for FBA Small And Light
 *
 * The Selling Partner API for FBA Small and Light lets you help sellers manage their listings in the Small and Light program. The program reduces the cost of fulfilling orders for small and lightweight FBA inventory. You can enroll or remove items from the program and check item eligibility and enrollment status. You can also preview the estimated program fees charged to a seller for items sold while enrolled in the program.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SmallAndLightEnrollment : The Small and Light enrollment status of the item indicated by the specified seller SKU.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallAndLightEnrollment {
  #[serde(rename = "marketplaceId")]
  marketplace_id: ::models::MarketplaceId,
  #[serde(rename = "sellerSKU")]
  seller_sku: ::models::SellerSku,
  #[serde(rename = "status")]
  status: ::models::SmallAndLightEnrollmentStatus
}

impl SmallAndLightEnrollment {
  /// The Small and Light enrollment status of the item indicated by the specified seller SKU.
  pub fn new(marketplace_id: ::models::MarketplaceId, seller_sku: ::models::SellerSku, status: ::models::SmallAndLightEnrollmentStatus) -> SmallAndLightEnrollment {
    SmallAndLightEnrollment {
      marketplace_id: marketplace_id,
      seller_sku: seller_sku,
      status: status
    }
  }

  pub fn set_marketplace_id(&mut self, marketplace_id: ::models::MarketplaceId) {
    self.marketplace_id = marketplace_id;
  }

  pub fn with_marketplace_id(mut self, marketplace_id: ::models::MarketplaceId) -> SmallAndLightEnrollment {
    self.marketplace_id = marketplace_id;
    self
  }

  pub fn marketplace_id(&self) -> &::models::MarketplaceId {
    &self.marketplace_id
  }


  pub fn set_seller_sku(&mut self, seller_sku: ::models::SellerSku) {
    self.seller_sku = seller_sku;
  }

  pub fn with_seller_sku(mut self, seller_sku: ::models::SellerSku) -> SmallAndLightEnrollment {
    self.seller_sku = seller_sku;
    self
  }

  pub fn seller_sku(&self) -> &::models::SellerSku {
    &self.seller_sku
  }


  pub fn set_status(&mut self, status: ::models::SmallAndLightEnrollmentStatus) {
    self.status = status;
  }

  pub fn with_status(mut self, status: ::models::SmallAndLightEnrollmentStatus) -> SmallAndLightEnrollment {
    self.status = status;
    self
  }

  pub fn status(&self) -> &::models::SmallAndLightEnrollmentStatus {
    &self.status
  }


}



