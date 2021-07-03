/* 
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * OpenAPI spec version: 2020-11-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StandardCompanyLogoModule : The standard company logo image.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StandardCompanyLogoModule {
  #[serde(rename = "companyLogo")]
  company_logo: ::models::ImageComponent
}

impl StandardCompanyLogoModule {
  /// The standard company logo image.
  pub fn new(company_logo: ::models::ImageComponent) -> StandardCompanyLogoModule {
    StandardCompanyLogoModule {
      company_logo: company_logo
    }
  }

  pub fn set_company_logo(&mut self, company_logo: ::models::ImageComponent) {
    self.company_logo = company_logo;
  }

  pub fn with_company_logo(mut self, company_logo: ::models::ImageComponent) -> StandardCompanyLogoModule {
    self.company_logo = company_logo;
    self
  }

  pub fn company_logo(&self) -> &::models::ImageComponent {
    &self.company_logo
  }


}



