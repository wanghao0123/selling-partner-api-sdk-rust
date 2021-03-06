/* 
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SafetReimbursementEvent : A SAFE-T claim reimbursement on the seller's account.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SafetReimbursementEvent {
  /// The date and time when the financial event was posted.
  #[serde(rename = "PostedDate")]
  posted_date: Option<::models::Date>,
  /// A SAFE-T claim identifier.
  #[serde(rename = "SAFETClaimId")]
  safet_claim_id: Option<String>,
  /// The amount of the reimbursement.
  #[serde(rename = "ReimbursedAmount")]
  reimbursed_amount: Option<::models::Currency>,
  /// Indicates why the seller was reimbursed.
  #[serde(rename = "ReasonCode")]
  reason_code: Option<String>,
  #[serde(rename = "SAFETReimbursementItemList")]
  safet_reimbursement_item_list: Option<::models::SafetReimbursementItemList>
}

impl SafetReimbursementEvent {
  /// A SAFE-T claim reimbursement on the seller's account.
  pub fn new() -> SafetReimbursementEvent {
    SafetReimbursementEvent {
      posted_date: None,
      safet_claim_id: None,
      reimbursed_amount: None,
      reason_code: None,
      safet_reimbursement_item_list: None
    }
  }

  pub fn set_posted_date(&mut self, posted_date: ::models::Date) {
    self.posted_date = Some(posted_date);
  }

  pub fn with_posted_date(mut self, posted_date: ::models::Date) -> SafetReimbursementEvent {
    self.posted_date = Some(posted_date);
    self
  }

  pub fn posted_date(&self) -> Option<&::models::Date> {
    self.posted_date.as_ref()
  }

  pub fn reset_posted_date(&mut self) {
    self.posted_date = None;
  }

  pub fn set_safet_claim_id(&mut self, safet_claim_id: String) {
    self.safet_claim_id = Some(safet_claim_id);
  }

  pub fn with_safet_claim_id(mut self, safet_claim_id: String) -> SafetReimbursementEvent {
    self.safet_claim_id = Some(safet_claim_id);
    self
  }

  pub fn safet_claim_id(&self) -> Option<&String> {
    self.safet_claim_id.as_ref()
  }

  pub fn reset_safet_claim_id(&mut self) {
    self.safet_claim_id = None;
  }

  pub fn set_reimbursed_amount(&mut self, reimbursed_amount: ::models::Currency) {
    self.reimbursed_amount = Some(reimbursed_amount);
  }

  pub fn with_reimbursed_amount(mut self, reimbursed_amount: ::models::Currency) -> SafetReimbursementEvent {
    self.reimbursed_amount = Some(reimbursed_amount);
    self
  }

  pub fn reimbursed_amount(&self) -> Option<&::models::Currency> {
    self.reimbursed_amount.as_ref()
  }

  pub fn reset_reimbursed_amount(&mut self) {
    self.reimbursed_amount = None;
  }

  pub fn set_reason_code(&mut self, reason_code: String) {
    self.reason_code = Some(reason_code);
  }

  pub fn with_reason_code(mut self, reason_code: String) -> SafetReimbursementEvent {
    self.reason_code = Some(reason_code);
    self
  }

  pub fn reason_code(&self) -> Option<&String> {
    self.reason_code.as_ref()
  }

  pub fn reset_reason_code(&mut self) {
    self.reason_code = None;
  }

  pub fn set_safet_reimbursement_item_list(&mut self, safet_reimbursement_item_list: ::models::SafetReimbursementItemList) {
    self.safet_reimbursement_item_list = Some(safet_reimbursement_item_list);
  }

  pub fn with_safet_reimbursement_item_list(mut self, safet_reimbursement_item_list: ::models::SafetReimbursementItemList) -> SafetReimbursementEvent {
    self.safet_reimbursement_item_list = Some(safet_reimbursement_item_list);
    self
  }

  pub fn safet_reimbursement_item_list(&self) -> Option<&::models::SafetReimbursementItemList> {
    self.safet_reimbursement_item_list.as_ref()
  }

  pub fn reset_safet_reimbursement_item_list(&mut self) {
    self.safet_reimbursement_item_list = None;
  }

}



