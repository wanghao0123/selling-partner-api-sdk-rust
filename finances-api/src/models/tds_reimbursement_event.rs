/* 
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// TdsReimbursementEvent : A tax deduction at source (TDS) claim reimbursement event on the seller's account.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TdsReimbursementEvent {
  /// The date and time when the financial event was posted.
  #[serde(rename = "PostedDate")]
  posted_date: Option<::models::Date>,
  /// A tax deduction at source (TDS) claim identifier.
  #[serde(rename = "TdsOrderId")]
  tds_order_id: Option<String>,
  /// The amount of the reimbursement.
  #[serde(rename = "ReimbursedAmount")]
  reimbursed_amount: Option<::models::Currency>
}

impl TdsReimbursementEvent {
  /// A tax deduction at source (TDS) claim reimbursement event on the seller's account.
  pub fn new() -> TdsReimbursementEvent {
    TdsReimbursementEvent {
      posted_date: None,
      tds_order_id: None,
      reimbursed_amount: None
    }
  }

  pub fn set_posted_date(&mut self, posted_date: ::models::Date) {
    self.posted_date = Some(posted_date);
  }

  pub fn with_posted_date(mut self, posted_date: ::models::Date) -> TdsReimbursementEvent {
    self.posted_date = Some(posted_date);
    self
  }

  pub fn posted_date(&self) -> Option<&::models::Date> {
    self.posted_date.as_ref()
  }

  pub fn reset_posted_date(&mut self) {
    self.posted_date = None;
  }

  pub fn set_tds_order_id(&mut self, tds_order_id: String) {
    self.tds_order_id = Some(tds_order_id);
  }

  pub fn with_tds_order_id(mut self, tds_order_id: String) -> TdsReimbursementEvent {
    self.tds_order_id = Some(tds_order_id);
    self
  }

  pub fn tds_order_id(&self) -> Option<&String> {
    self.tds_order_id.as_ref()
  }

  pub fn reset_tds_order_id(&mut self) {
    self.tds_order_id = None;
  }

  pub fn set_reimbursed_amount(&mut self, reimbursed_amount: ::models::Currency) {
    self.reimbursed_amount = Some(reimbursed_amount);
  }

  pub fn with_reimbursed_amount(mut self, reimbursed_amount: ::models::Currency) -> TdsReimbursementEvent {
    self.reimbursed_amount = Some(reimbursed_amount);
    self
  }

  pub fn reimbursed_amount(&self) -> Option<&::models::Currency> {
    self.reimbursed_amount.as_ref()
  }

  pub fn reset_reimbursed_amount(&mut self) {
    self.reimbursed_amount = None;
  }

}



