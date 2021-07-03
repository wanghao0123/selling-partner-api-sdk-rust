/* 
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * OpenAPI spec version: 2020-11-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StandardImageCaptionBlock : The A+ Content standard image and caption block.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StandardImageCaptionBlock {
  #[serde(rename = "image")]
  image: Option<::models::ImageComponent>,
  #[serde(rename = "caption")]
  caption: Option<::models::TextComponent>
}

impl StandardImageCaptionBlock {
  /// The A+ Content standard image and caption block.
  pub fn new() -> StandardImageCaptionBlock {
    StandardImageCaptionBlock {
      image: None,
      caption: None
    }
  }

  pub fn set_image(&mut self, image: ::models::ImageComponent) {
    self.image = Some(image);
  }

  pub fn with_image(mut self, image: ::models::ImageComponent) -> StandardImageCaptionBlock {
    self.image = Some(image);
    self
  }

  pub fn image(&self) -> Option<&::models::ImageComponent> {
    self.image.as_ref()
  }

  pub fn reset_image(&mut self) {
    self.image = None;
  }

  pub fn set_caption(&mut self, caption: ::models::TextComponent) {
    self.caption = Some(caption);
  }

  pub fn with_caption(mut self, caption: ::models::TextComponent) -> StandardImageCaptionBlock {
    self.caption = Some(caption);
    self
  }

  pub fn caption(&self) -> Option<&::models::TextComponent> {
    self.caption.as_ref()
  }

  pub fn reset_caption(&mut self) {
    self.caption = None;
  }

}



