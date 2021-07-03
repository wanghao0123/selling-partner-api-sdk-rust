/* 
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// LoanServicingEvent : A loan advance, loan payment, or loan refund.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoanServicingEvent {
  /// The amount of the loan.
  #[serde(rename = "LoanAmount")]
  loan_amount: Option<::models::Currency>,
  /// The type of event.  Possible values:  * LoanAdvance  * LoanPayment  * LoanRefund
  #[serde(rename = "SourceBusinessEventType")]
  source_business_event_type: Option<String>
}

impl LoanServicingEvent {
  /// A loan advance, loan payment, or loan refund.
  pub fn new() -> LoanServicingEvent {
    LoanServicingEvent {
      loan_amount: None,
      source_business_event_type: None
    }
  }

  pub fn set_loan_amount(&mut self, loan_amount: ::models::Currency) {
    self.loan_amount = Some(loan_amount);
  }

  pub fn with_loan_amount(mut self, loan_amount: ::models::Currency) -> LoanServicingEvent {
    self.loan_amount = Some(loan_amount);
    self
  }

  pub fn loan_amount(&self) -> Option<&::models::Currency> {
    self.loan_amount.as_ref()
  }

  pub fn reset_loan_amount(&mut self) {
    self.loan_amount = None;
  }

  pub fn set_source_business_event_type(&mut self, source_business_event_type: String) {
    self.source_business_event_type = Some(source_business_event_type);
  }

  pub fn with_source_business_event_type(mut self, source_business_event_type: String) -> LoanServicingEvent {
    self.source_business_event_type = Some(source_business_event_type);
    self
  }

  pub fn source_business_event_type(&self) -> Option<&String> {
    self.source_business_event_type.as_ref()
  }

  pub fn reset_source_business_event_type(&mut self) {
    self.source_business_event_type = None;
  }

}



