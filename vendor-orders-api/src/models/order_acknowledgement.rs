/* 
 * Selling Partner API for Retail Procurement Orders
 *
 * The Selling Partner API for Retail Procurement Orders provides programmatic access to vendor orders data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderAcknowledgement {
  /// The purchase order number. Formatting Notes: 8-character alpha-numeric code.
  #[serde(rename = "purchaseOrderNumber")]
  purchase_order_number: String,
  /// Name, address and tax details of the party receiving a shipment of products.
  #[serde(rename = "sellingParty")]
  selling_party: ::models::PartyIdentification,
  /// The date and time when the purchase order is acknowledged, in ISO-8601 date/time format.
  #[serde(rename = "acknowledgementDate")]
  acknowledgement_date: String,
  /// A list of the items being acknowledged with associated details.
  #[serde(rename = "items")]
  items: Vec<::models::OrderAcknowledgementItem>
}

impl OrderAcknowledgement {
  pub fn new(purchase_order_number: String, selling_party: ::models::PartyIdentification, acknowledgement_date: String, items: Vec<::models::OrderAcknowledgementItem>) -> OrderAcknowledgement {
    OrderAcknowledgement {
      purchase_order_number: purchase_order_number,
      selling_party: selling_party,
      acknowledgement_date: acknowledgement_date,
      items: items
    }
  }

  pub fn set_purchase_order_number(&mut self, purchase_order_number: String) {
    self.purchase_order_number = purchase_order_number;
  }

  pub fn with_purchase_order_number(mut self, purchase_order_number: String) -> OrderAcknowledgement {
    self.purchase_order_number = purchase_order_number;
    self
  }

  pub fn purchase_order_number(&self) -> &String {
    &self.purchase_order_number
  }


  pub fn set_selling_party(&mut self, selling_party: ::models::PartyIdentification) {
    self.selling_party = selling_party;
  }

  pub fn with_selling_party(mut self, selling_party: ::models::PartyIdentification) -> OrderAcknowledgement {
    self.selling_party = selling_party;
    self
  }

  pub fn selling_party(&self) -> &::models::PartyIdentification {
    &self.selling_party
  }


  pub fn set_acknowledgement_date(&mut self, acknowledgement_date: String) {
    self.acknowledgement_date = acknowledgement_date;
  }

  pub fn with_acknowledgement_date(mut self, acknowledgement_date: String) -> OrderAcknowledgement {
    self.acknowledgement_date = acknowledgement_date;
    self
  }

  pub fn acknowledgement_date(&self) -> &String {
    &self.acknowledgement_date
  }


  pub fn set_items(&mut self, items: Vec<::models::OrderAcknowledgementItem>) {
    self.items = items;
  }

  pub fn with_items(mut self, items: Vec<::models::OrderAcknowledgementItem>) -> OrderAcknowledgement {
    self.items = items;
    self
  }

  pub fn items(&self) -> &Vec<::models::OrderAcknowledgementItem> {
    &self.items
  }


}



