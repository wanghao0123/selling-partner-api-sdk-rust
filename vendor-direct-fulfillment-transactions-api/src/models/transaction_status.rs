/* 
 * Selling Partner API for Direct Fulfillment Transaction Status
 *
 * The Selling Partner API for Direct Fulfillment Transaction Status provides programmatic access to a direct fulfillment vendor's transaction status.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// TransactionStatus : The payload for the getTransactionStatus operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionStatus {
  #[serde(rename = "transactionStatus")]
  transaction_status: Option<::models::Transaction>
}

impl TransactionStatus {
  /// The payload for the getTransactionStatus operation.
  pub fn new() -> TransactionStatus {
    TransactionStatus {
      transaction_status: None
    }
  }

  pub fn set_transaction_status(&mut self, transaction_status: ::models::Transaction) {
    self.transaction_status = Some(transaction_status);
  }

  pub fn with_transaction_status(mut self, transaction_status: ::models::Transaction) -> TransactionStatus {
    self.transaction_status = Some(transaction_status);
    self
  }

  pub fn transaction_status(&self) -> Option<&::models::Transaction> {
    self.transaction_status.as_ref()
  }

  pub fn reset_transaction_status(&mut self) {
    self.transaction_status = None;
  }

}



