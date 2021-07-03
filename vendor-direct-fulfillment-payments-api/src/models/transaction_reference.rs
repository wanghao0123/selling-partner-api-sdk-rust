/* 
 * Selling Partner API for Direct Fulfillment Payments
 *
 * The Selling Partner API for Direct Fulfillment Payments provides programmatic access to a direct fulfillment vendor's invoice data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionReference {
  /// GUID to identify this transaction. This value can be used with the Transaction Status API to return the status of this transaction.
  #[serde(rename = "transactionId")]
  transaction_id: Option<String>
}

impl TransactionReference {
  pub fn new() -> TransactionReference {
    TransactionReference {
      transaction_id: None
    }
  }

  pub fn set_transaction_id(&mut self, transaction_id: String) {
    self.transaction_id = Some(transaction_id);
  }

  pub fn with_transaction_id(mut self, transaction_id: String) -> TransactionReference {
    self.transaction_id = Some(transaction_id);
    self
  }

  pub fn transaction_id(&self) -> Option<&String> {
    self.transaction_id.as_ref()
  }

  pub fn reset_transaction_id(&mut self) {
    self.transaction_id = None;
  }

}



