/* 
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * OpenAPI spec version: 2020-11-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StandardThreeImageTextModule : Three standard images with text, presented across a single row.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StandardThreeImageTextModule {
  #[serde(rename = "headline")]
  headline: Option<::models::TextComponent>,
  #[serde(rename = "block1")]
  block1: Option<::models::StandardImageTextBlock>,
  #[serde(rename = "block2")]
  block2: Option<::models::StandardImageTextBlock>,
  #[serde(rename = "block3")]
  block3: Option<::models::StandardImageTextBlock>
}

impl StandardThreeImageTextModule {
  /// Three standard images with text, presented across a single row.
  pub fn new() -> StandardThreeImageTextModule {
    StandardThreeImageTextModule {
      headline: None,
      block1: None,
      block2: None,
      block3: None
    }
  }

  pub fn set_headline(&mut self, headline: ::models::TextComponent) {
    self.headline = Some(headline);
  }

  pub fn with_headline(mut self, headline: ::models::TextComponent) -> StandardThreeImageTextModule {
    self.headline = Some(headline);
    self
  }

  pub fn headline(&self) -> Option<&::models::TextComponent> {
    self.headline.as_ref()
  }

  pub fn reset_headline(&mut self) {
    self.headline = None;
  }

  pub fn set_block1(&mut self, block1: ::models::StandardImageTextBlock) {
    self.block1 = Some(block1);
  }

  pub fn with_block1(mut self, block1: ::models::StandardImageTextBlock) -> StandardThreeImageTextModule {
    self.block1 = Some(block1);
    self
  }

  pub fn block1(&self) -> Option<&::models::StandardImageTextBlock> {
    self.block1.as_ref()
  }

  pub fn reset_block1(&mut self) {
    self.block1 = None;
  }

  pub fn set_block2(&mut self, block2: ::models::StandardImageTextBlock) {
    self.block2 = Some(block2);
  }

  pub fn with_block2(mut self, block2: ::models::StandardImageTextBlock) -> StandardThreeImageTextModule {
    self.block2 = Some(block2);
    self
  }

  pub fn block2(&self) -> Option<&::models::StandardImageTextBlock> {
    self.block2.as_ref()
  }

  pub fn reset_block2(&mut self) {
    self.block2 = None;
  }

  pub fn set_block3(&mut self, block3: ::models::StandardImageTextBlock) {
    self.block3 = Some(block3);
  }

  pub fn with_block3(mut self, block3: ::models::StandardImageTextBlock) -> StandardThreeImageTextModule {
    self.block3 = Some(block3);
    self
  }

  pub fn block3(&self) -> Option<&::models::StandardImageTextBlock> {
    self.block3.as_ref()
  }

  pub fn reset_block3(&mut self) {
    self.block3 = None;
  }

}



