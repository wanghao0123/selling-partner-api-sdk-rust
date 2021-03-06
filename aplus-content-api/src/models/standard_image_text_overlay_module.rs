/* 
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * OpenAPI spec version: 2020-11-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StandardImageTextOverlayModule : A standard background image with a floating text box.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StandardImageTextOverlayModule {
  #[serde(rename = "overlayColorType")]
  overlay_color_type: ::models::ColorType,
  #[serde(rename = "block")]
  block: Option<::models::StandardImageTextBlock>
}

impl StandardImageTextOverlayModule {
  /// A standard background image with a floating text box.
  pub fn new(overlay_color_type: ::models::ColorType) -> StandardImageTextOverlayModule {
    StandardImageTextOverlayModule {
      overlay_color_type: overlay_color_type,
      block: None
    }
  }

  pub fn set_overlay_color_type(&mut self, overlay_color_type: ::models::ColorType) {
    self.overlay_color_type = overlay_color_type;
  }

  pub fn with_overlay_color_type(mut self, overlay_color_type: ::models::ColorType) -> StandardImageTextOverlayModule {
    self.overlay_color_type = overlay_color_type;
    self
  }

  pub fn overlay_color_type(&self) -> &::models::ColorType {
    &self.overlay_color_type
  }


  pub fn set_block(&mut self, block: ::models::StandardImageTextBlock) {
    self.block = Some(block);
  }

  pub fn with_block(mut self, block: ::models::StandardImageTextBlock) -> StandardImageTextOverlayModule {
    self.block = Some(block);
    self
  }

  pub fn block(&self) -> Option<&::models::StandardImageTextBlock> {
    self.block.as_ref()
  }

  pub fn reset_block(&mut self) {
    self.block = None;
  }

}



