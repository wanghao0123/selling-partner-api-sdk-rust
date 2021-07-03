/* 
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * OpenAPI spec version: 2020-11-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StandardHeaderImageTextModule : Standard headline text, an image, and body text.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StandardHeaderImageTextModule {
  #[serde(rename = "headline")]
  headline: Option<::models::TextComponent>,
  #[serde(rename = "block")]
  block: Option<::models::StandardImageTextBlock>
}

impl StandardHeaderImageTextModule {
  /// Standard headline text, an image, and body text.
  pub fn new() -> StandardHeaderImageTextModule {
    StandardHeaderImageTextModule {
      headline: None,
      block: None
    }
  }

  pub fn set_headline(&mut self, headline: ::models::TextComponent) {
    self.headline = Some(headline);
  }

  pub fn with_headline(mut self, headline: ::models::TextComponent) -> StandardHeaderImageTextModule {
    self.headline = Some(headline);
    self
  }

  pub fn headline(&self) -> Option<&::models::TextComponent> {
    self.headline.as_ref()
  }

  pub fn reset_headline(&mut self) {
    self.headline = None;
  }

  pub fn set_block(&mut self, block: ::models::StandardImageTextBlock) {
    self.block = Some(block);
  }

  pub fn with_block(mut self, block: ::models::StandardImageTextBlock) -> StandardHeaderImageTextModule {
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



