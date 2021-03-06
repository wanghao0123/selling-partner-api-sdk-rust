/* 
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DebtRecoveryItem : An item of a debt payment or debt adjustment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DebtRecoveryItem {
  /// The amount applied for the recovery item.
  #[serde(rename = "RecoveryAmount")]
  recovery_amount: Option<::models::Currency>,
  /// The original debt amount.
  #[serde(rename = "OriginalAmount")]
  original_amount: Option<::models::Currency>,
  /// The beginning date and time of the financial event group that contains the debt. In ISO 8601 date time format.
  #[serde(rename = "GroupBeginDate")]
  group_begin_date: Option<::models::Date>,
  /// The ending date and time of the financial event group that contains the debt. In ISO 8601 date time format.
  #[serde(rename = "GroupEndDate")]
  group_end_date: Option<::models::Date>
}

impl DebtRecoveryItem {
  /// An item of a debt payment or debt adjustment.
  pub fn new() -> DebtRecoveryItem {
    DebtRecoveryItem {
      recovery_amount: None,
      original_amount: None,
      group_begin_date: None,
      group_end_date: None
    }
  }

  pub fn set_recovery_amount(&mut self, recovery_amount: ::models::Currency) {
    self.recovery_amount = Some(recovery_amount);
  }

  pub fn with_recovery_amount(mut self, recovery_amount: ::models::Currency) -> DebtRecoveryItem {
    self.recovery_amount = Some(recovery_amount);
    self
  }

  pub fn recovery_amount(&self) -> Option<&::models::Currency> {
    self.recovery_amount.as_ref()
  }

  pub fn reset_recovery_amount(&mut self) {
    self.recovery_amount = None;
  }

  pub fn set_original_amount(&mut self, original_amount: ::models::Currency) {
    self.original_amount = Some(original_amount);
  }

  pub fn with_original_amount(mut self, original_amount: ::models::Currency) -> DebtRecoveryItem {
    self.original_amount = Some(original_amount);
    self
  }

  pub fn original_amount(&self) -> Option<&::models::Currency> {
    self.original_amount.as_ref()
  }

  pub fn reset_original_amount(&mut self) {
    self.original_amount = None;
  }

  pub fn set_group_begin_date(&mut self, group_begin_date: ::models::Date) {
    self.group_begin_date = Some(group_begin_date);
  }

  pub fn with_group_begin_date(mut self, group_begin_date: ::models::Date) -> DebtRecoveryItem {
    self.group_begin_date = Some(group_begin_date);
    self
  }

  pub fn group_begin_date(&self) -> Option<&::models::Date> {
    self.group_begin_date.as_ref()
  }

  pub fn reset_group_begin_date(&mut self) {
    self.group_begin_date = None;
  }

  pub fn set_group_end_date(&mut self, group_end_date: ::models::Date) {
    self.group_end_date = Some(group_end_date);
  }

  pub fn with_group_end_date(mut self, group_end_date: ::models::Date) -> DebtRecoveryItem {
    self.group_end_date = Some(group_end_date);
    self
  }

  pub fn group_end_date(&self) -> Option<&::models::Date> {
    self.group_end_date.as_ref()
  }

  pub fn reset_group_end_date(&mut self) {
    self.group_end_date = None;
  }

}



